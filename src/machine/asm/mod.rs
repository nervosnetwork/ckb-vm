use crate::{
    decoder::{build_decoder, Decoder},
    instructions::{
        blank_instruction, execute, execute_instruction, execute_nocheck, extract_opcode,
        generate_handle_function_list, generate_vcheck_function_list, instruction_length,
        is_basic_block_end_instruction, is_slowpath_instruction, is_slowpath_opcode, Instruction,
    },
    machine::{CoprocessorV, VERSION0},
    memory::{
        fill_page_data, get_page_indices, memset, round_page_down, round_page_up, FLAG_DIRTY,
        FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT,
    },
    CoreMachine, DefaultMachine, Error, Machine, Memory, SupportMachine, MEMORY_FRAME_SHIFTS,
    RISCV_MAX_MEMORY, RISCV_PAGES, RISCV_PAGESIZE,
};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
pub use ckb_vm_definitions::asm::AsmCoreMachine;
use ckb_vm_definitions::{
    asm::{
        calculate_slot, Trace, RET_CYCLES_OVERFLOW, RET_DECODE_TRACE, RET_DYNAMIC_JUMP, RET_EBREAK,
        RET_ECALL, RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND, RET_SLOWPATH,
        RET_SLOWPATH_TRACE, TRACE_ITEM_LENGTH, TRACE_SIZE,
    },
    instructions::{OP_CUSTOM_TRACE_END, OP_VSETIVLI, OP_VSETVL, OP_VSETVLI},
    ISA_MOP, MEMORY_FRAMES, MEMORY_FRAME_PAGE_SHIFTS, RISCV_GENERAL_REGISTER_NUMBER,
    RISCV_PAGE_SHIFTS,
};
use libc::c_uchar;
use memmap::Mmap;
use probe::probe;
use rand::{prelude::RngCore, SeedableRng};
use std::collections::HashMap;
use std::mem::transmute;
use std::sync::Arc;

pub struct AsmGlueMachine {
    pub imc: Box<AsmCoreMachine>,
    pub rvv: CoprocessorV,
}

impl AsmGlueMachine {
    pub fn new(machine: Box<AsmCoreMachine>) -> Self {
        Self {
            imc: machine,
            rvv: CoprocessorV::new(),
        }
    }
}

impl CoreMachine for AsmGlueMachine {
    type REG = u64;
    type MEM = Self;

    fn pc(&self) -> &Self::REG {
        &self.imc.pc
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.imc.next_pc = pc;
    }

    fn commit_pc(&mut self) {
        self.imc.pc = self.imc.next_pc;
    }

    fn memory(&self) -> &Self {
        self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
    }

    fn registers(&self) -> &[Self::REG] {
        &self.imc.registers
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.imc.registers[idx] = value;
    }

    fn coprocessor_v(&self) -> &CoprocessorV {
        &self.rvv
    }

    fn coprocessor_v_mut(&mut self) -> &mut CoprocessorV {
        &mut self.rvv
    }

    fn isa(&self) -> u8 {
        self.imc.isa
    }

    fn version(&self) -> u32 {
        self.imc.version
    }
}

// This function is exported for asm and aot machine.
// Note that the parameter `machine` is after parameter `frame_index`. Generally
// speaking, put `machine` in the first parameter is more human readable,
// but consider that in the asm machine, `frame_index` is stored in `rdi` and `machine`
// is stored in `rsi`, there is no need to exchange the values in the two registers
// in this way.
#[no_mangle]
pub extern "C" fn inited_memory(frame_index: u64, machine: &mut AsmCoreMachine) {
    let addr_from = (frame_index << MEMORY_FRAME_SHIFTS) as usize;
    let addr_to = ((frame_index + 1) << MEMORY_FRAME_SHIFTS) as usize;
    if machine.chaos_mode != 0 {
        let mut gen = rand::rngs::StdRng::seed_from_u64(machine.chaos_seed.into());
        gen.fill_bytes(&mut machine.memory[addr_from..addr_to]);
        machine.chaos_seed = gen.next_u32();
    } else {
        memset(&mut machine.memory[addr_from..addr_to], 0);
    }
}

pub(crate) fn check_memory(machine: &mut AsmCoreMachine, page: u64) {
    let frame = page >> MEMORY_FRAME_PAGE_SHIFTS;
    if machine.frames[frame as usize] == 0 {
        inited_memory(frame, machine);
        machine.frames[frame as usize] = 1;
    }
}

pub(crate) fn check_permission<M: Memory>(
    memory: &mut M,
    page: u64,
    flag: u8,
) -> Result<(), Error> {
    let page_flag = memory.fetch_flag(page)?;
    if (page_flag & FLAG_WXORX_BIT) != (flag & FLAG_WXORX_BIT) {
        return Err(Error::MemWriteOnExecutablePage);
    }
    Ok(())
}

// check whether a memory address is writable or not and mark it as dirty, `size` should be 1, 2, 4 or 8
fn check_memory_writable(
    machine: &mut Box<AsmCoreMachine>,
    addr: u64,
    size: usize,
) -> Result<(), Error> {
    debug_assert!(size == 1 || size == 2 || size == 4 || size == 8);
    let page = addr >> RISCV_PAGE_SHIFTS;
    if page as usize >= RISCV_PAGES {
        return Err(Error::MemOutOfBound);
    }
    check_permission(machine, page, FLAG_WRITABLE)?;
    check_memory(machine, page);
    machine.set_flag(page, FLAG_DIRTY)?;

    // check next page if neccessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= RISCV_PAGES {
            return Err(Error::MemOutOfBound);
        } else {
            check_permission(machine, page, FLAG_WRITABLE)?;
            check_memory(machine, page);
            machine.set_flag(page, FLAG_DIRTY)?
        }
    }
    Ok(())
}

// check whether a memory address is executable, `size` should be 2 or 4
fn check_memory_executable(
    machine: &mut Box<AsmCoreMachine>,
    addr: u64,
    size: usize,
) -> Result<(), Error> {
    debug_assert!(size == 2 || size == 4);

    let page = addr >> RISCV_PAGE_SHIFTS;
    if page as usize >= RISCV_PAGES {
        return Err(Error::MemOutOfBound);
    }
    check_permission(machine, page, FLAG_EXECUTABLE)?;
    check_memory(machine, page);

    // check next page if neccessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= RISCV_PAGES {
            return Err(Error::MemOutOfBound);
        } else {
            check_permission(machine, page, FLAG_EXECUTABLE)?;
            check_memory(machine, page);
        }
    }
    Ok(())
}

// check whether a memory address is initialized, `size` should be le RISCV_PAGESIZE
pub(crate) fn check_memory_inited(
    machine: &mut Box<AsmCoreMachine>,
    addr: u64,
    size: usize,
) -> Result<(), Error> {
    debug_assert!(size <= RISCV_PAGESIZE);
    let page = addr >> RISCV_PAGE_SHIFTS;
    if page as usize >= RISCV_PAGES {
        return Err(Error::MemOutOfBound);
    }
    check_memory(machine, page);

    // check next page if neccessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= RISCV_PAGES {
            return Err(Error::MemOutOfBound);
        } else {
            check_memory(machine, page);
        }
    }
    Ok(())
}

impl Memory for Box<AsmCoreMachine> {
    type REG = u64;

    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error> {
        if round_page_down(addr) != addr || round_page_up(size) != size {
            return Err(Error::MemPageUnalignedAccess);
        }
        if addr > RISCV_MAX_MEMORY as u64
            || size > RISCV_MAX_MEMORY as u64
            || addr + size > RISCV_MAX_MEMORY as u64
            || offset_from_addr > size
        {
            return Err(Error::MemOutOfBound);
        }
        // We benchmarked the code piece here, using while loop this way is
        // actually faster than a for..in solution. The difference is roughly
        // 3% so we are keeping this version.
        let mut current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            if self.fetch_flag(page)? & FLAG_FREEZED != 0 {
                return Err(Error::MemWriteOnFreezedPage);
            }
            current_addr += RISCV_PAGESIZE as u64;
        }
        fill_page_data(self, addr, size, source, offset_from_addr)?;
        current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            self.set_flag(page, flags)?;
            current_addr += RISCV_PAGESIZE as u64;
        }
        Ok(())
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        if page < RISCV_PAGES as u64 {
            Ok(self.flags[page as usize])
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < RISCV_PAGES as u64 {
            self.flags[page as usize] |= flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < RISCV_PAGES as u64 {
            self.flags[page as usize] &= !flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, value.len() as u64)?;
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        let slice = &mut self.memory[addr as usize..addr as usize + value.len()];
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, size)?;
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        memset(
            &mut self.memory[addr as usize..(addr + size) as usize],
            value,
        );
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Vec<u8>, Error> {
        check_memory_inited(self, addr, size as usize)?;
        Ok(self.memory[addr as usize..(addr + size) as usize].to_vec())
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        check_memory_executable(self, addr, 2)?;
        Ok(LittleEndian::read_u16(
            &self.memory[addr as usize..addr as usize + 2],
        ))
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        check_memory_executable(self, addr, 4)?;
        Ok(LittleEndian::read_u32(
            &self.memory[addr as usize..addr as usize + 4],
        ))
    }

    fn load8(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 1)?;
        Ok(u64::from(self.memory[addr as usize]))
    }

    fn load16(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 2)?;
        Ok(u64::from(LittleEndian::read_u16(
            &self.memory[addr as usize..addr as usize + 2],
        )))
    }

    fn load32(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 4)?;
        Ok(u64::from(LittleEndian::read_u32(
            &self.memory[addr as usize..addr as usize + 4],
        )))
    }

    fn load64(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 8)?;
        Ok(LittleEndian::read_u64(
            &self.memory[addr as usize..addr as usize + 8],
        ))
    }

    fn store8(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 1)?;
        self.memory[addr as usize] = (*value) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 2)?;
        LittleEndian::write_u16(
            &mut self.memory[addr as usize..(addr + 2) as usize],
            *value as u16,
        );
        Ok(())
    }

    fn store32(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 4)?;
        LittleEndian::write_u32(
            &mut self.memory[addr as usize..(addr + 4) as usize],
            *value as u32,
        );
        Ok(())
    }

    fn store64(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 8)?;
        LittleEndian::write_u64(&mut self.memory[addr as usize..(addr + 8) as usize], *value);
        Ok(())
    }
}

impl Memory for AsmGlueMachine {
    type REG = u64;

    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error> {
        self.imc
            .init_pages(addr, size, flags, source, offset_from_addr)
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        self.imc.fetch_flag(page)
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        self.imc.set_flag(page, flag)
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        self.imc.clear_flag(page, flag)
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        self.imc.store_bytes(addr, value)
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        self.imc.store_byte(addr, size, value)
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Vec<u8>, Error> {
        self.imc.load_bytes(addr, size)
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        self.imc.execute_load16(addr)
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        self.imc.execute_load32(addr)
    }

    fn load8(&mut self, addr: &u64) -> Result<u64, Error> {
        self.imc.load8(addr)
    }

    fn load16(&mut self, addr: &u64) -> Result<u64, Error> {
        self.imc.load16(addr)
    }

    fn load32(&mut self, addr: &u64) -> Result<u64, Error> {
        self.imc.load32(addr)
    }

    fn load64(&mut self, addr: &u64) -> Result<u64, Error> {
        self.imc.load64(addr)
    }

    fn store8(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        self.imc.store8(addr, value)
    }

    fn store16(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        self.imc.store16(addr, value)
    }

    fn store32(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        self.imc.store32(addr, value)
    }

    fn store64(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        self.imc.store64(addr, value)
    }
}

impl SupportMachine for AsmGlueMachine {
    fn cycles(&self) -> u64 {
        self.imc.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.imc.cycles = cycles;
    }

    fn max_cycles(&self) -> u64 {
        self.imc.max_cycles
    }

    fn reset(&mut self, max_cycles: u64) {
        self.imc.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
        self.imc.pc = 0;
        self.imc.flags = [0; RISCV_PAGES];
        for i in 0..TRACE_SIZE {
            self.imc.traces[i] = Trace::default();
        }
        self.imc.frames = [0; MEMORY_FRAMES];
        self.imc.cycles = 0;
        self.imc.max_cycles = max_cycles;
        self.imc.reset_signal = 1;
    }

    fn reset_signal(&mut self) -> bool {
        let ret = self.imc.reset_signal != 0;
        self.imc.reset_signal = 0;
        ret
    }

    fn running(&self) -> bool {
        self.imc.running == 1
    }

    fn set_running(&mut self, running: bool) {
        self.imc.running = if running { 1 } else { 0 }
    }

    #[cfg(feature = "pprof")]
    fn code(&self) -> &Bytes {
        unreachable!()
    }
}

#[derive(Debug)]
pub enum VTraceType {
    StableCheckAndCycles,
    StableCheck,
    Unstable,
}

pub fn vtrace_type(instructions: &[Instruction]) -> VTraceType {
    let mut vfirst = 0;
    for instruction in instructions {
        let op = extract_opcode(*instruction);
        if op == OP_CUSTOM_TRACE_END {
            break;
        }
        if is_slowpath_opcode(op) {
            if vfirst == 0 {
                if op != OP_VSETIVLI && op != OP_VSETVLI && op != OP_VSETVL {
                    return VTraceType::Unstable;
                }
                vfirst = op
            } else {
                if op == OP_VSETIVLI || op == OP_VSETVLI || op == OP_VSETVL {
                    return VTraceType::Unstable;
                }
            }
        }
    }
    if vfirst == OP_VSETIVLI {
        return VTraceType::StableCheckAndCycles;
    }
    if vfirst == OP_VSETVLI {
        return VTraceType::StableCheck;
    }
    return VTraceType::Unstable;
}

pub struct AotCode {
    pub code: Mmap,
    /// Labels that map RISC-V addresses to offsets into the compiled x86_64
    /// assembly code. This can be used as entrypoints to start executing in
    /// AOT code.
    pub labels: HashMap<u64, u32>,
}

impl AotCode {
    pub fn base_address(&self) -> u64 {
        self.code.as_ptr() as u64
    }
}

pub struct AsmMachine {
    pub machine: DefaultMachine<AsmGlueMachine>,
    pub aot_code: Option<Arc<AotCode>>,
}

extern "C" {
    pub fn ckb_vm_x64_execute(m: *mut AsmCoreMachine) -> c_uchar;
    // We are keeping this as a function here, but at the bottom level this really
    // just points to an array of assembly label offsets for each opcode.
    pub fn ckb_vm_asm_labels();
}

impl AsmMachine {
    pub fn new(machine: DefaultMachine<AsmGlueMachine>, aot_code: Option<Arc<AotCode>>) -> Self {
        Self { machine, aot_code }
    }

    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.machine.inner.imc.max_cycles = cycles;
    }

    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }

    pub fn run(&mut self) -> Result<i8, Error> {
        if self.machine.isa() & ISA_MOP != 0 && self.machine.version() == VERSION0 {
            return Err(Error::InvalidVersion);
        }
        let mut decoder = build_decoder::<u64>(self.machine.isa(), self.machine.version());
        let vcheck_function_list =
            generate_vcheck_function_list::<DefaultMachine<AsmGlueMachine>>();
        let handle_function_list =
            generate_handle_function_list::<DefaultMachine<AsmGlueMachine>>();
        self.machine.set_running(true);
        while self.machine.running() {
            if self.machine.reset_signal() {
                decoder.reset_instructions_cache();
                self.aot_code = None;
            }
            let result = if let Some(aot_code) = &self.aot_code {
                if let Some(offset) = aot_code.labels.get(self.machine.pc()) {
                    let base_address = aot_code.base_address();
                    let offset_address = base_address + u64::from(*offset);
                    let f = unsafe {
                        transmute::<u64, fn(*mut AsmCoreMachine, u64) -> u8>(base_address)
                    };
                    f(&mut *(self.machine.inner_mut().imc), offset_address)
                } else {
                    unsafe { ckb_vm_x64_execute(&mut *(self.machine.inner_mut().imc)) }
                }
            } else {
                unsafe { ckb_vm_x64_execute(&mut *(self.machine.inner_mut().imc)) }
            };
            match result {
                RET_DECODE_TRACE => {
                    probe!(default, decode_trace_begin);
                    let pc = *self.machine.pc();
                    let slot = calculate_slot(pc);
                    let mut trace = Trace::default();
                    let mut current_pc = pc;
                    let mut i = 0;
                    while i < TRACE_ITEM_LENGTH {
                        let instruction = decoder.decode(self.machine.memory_mut(), current_pc)?;
                        let end_instruction = is_basic_block_end_instruction(instruction);
                        let is_slowpath = is_slowpath_instruction(instruction);
                        current_pc += u64::from(instruction_length(instruction));
                        if trace.slowpath == 0 && is_slowpath {
                            trace.slowpath = 1;
                        }
                        trace.instructions[i] = instruction;
                        // don't count cycles in trace for RVV instructions. They
                        // will be counted in slow path.
                        if !is_slowpath {
                            trace.cycles +=
                                self.machine.instruction_cycle_func()(instruction, 0, 0);
                        }
                        let opcode = extract_opcode(instruction);
                        // Here we are calculating the absolute address used in direct threading
                        // from label offsets.
                        trace.thread[i] = unsafe {
                            u64::from(
                                *(ckb_vm_asm_labels as *const u32).offset(opcode as u8 as isize),
                            ) + (ckb_vm_asm_labels as *const u32 as u64)
                        };
                        i += 1;
                        if end_instruction {
                            break;
                        }
                    }
                    trace.instructions[i] = blank_instruction(OP_CUSTOM_TRACE_END);
                    trace.thread[i] = unsafe {
                        u64::from(
                            *(ckb_vm_asm_labels as *const u32).offset(OP_CUSTOM_TRACE_END as isize),
                        ) + (ckb_vm_asm_labels as *const u32 as u64)
                    };
                    trace.address = pc;
                    trace.length = (current_pc - pc) as u8;
                    self.machine.inner_mut().imc.traces[slot] = trace;
                    probe!(default, trace_instruction_count, i as isize);
                }
                RET_ECALL => self.machine.ecall()?,
                RET_EBREAK => self.machine.ebreak()?,
                RET_DYNAMIC_JUMP => (),
                RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
                RET_CYCLES_OVERFLOW => return Err(Error::CyclesOverflow),
                RET_OUT_OF_BOUND => return Err(Error::MemOutOfBound),
                RET_INVALID_PERMISSION => return Err(Error::MemWriteOnExecutablePage),
                RET_SLOWPATH => {
                    let pc = *self.machine.pc() - 4;
                    let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
                    let cycles = self.machine.instruction_cycle_func()(
                        instruction,
                        self.machine.inner.rvv.vl(),
                        self.machine.inner.rvv.vsew(),
                    );
                    self.machine.add_cycles(cycles)?;
                    let op = extract_opcode(instruction);
                    vcheck_function_list[op as usize](&mut self.machine, instruction)?;
                    handle_function_list[op as usize](&mut self.machine, instruction)?;
                    probe!(default, slow_path, cycles as isize);
                }
                RET_SLOWPATH_TRACE => {
                    let pc = *self.machine.pc();
                    let slot = calculate_slot(pc);
                    let slowpath = self.machine.inner_mut().imc.traces[slot].slowpath;
                    match slowpath {
                        0b101 => {
                            let cycles = self.machine.inner_mut().imc.traces[slot].cycles;
                            self.machine.add_cycles(cycles)?;
                            for instruction in
                                self.machine.inner_mut().imc.traces[slot].instructions
                            {
                                if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                                    break;
                                }
                                execute_nocheck(
                                    &mut self.machine,
                                    &handle_function_list,
                                    instruction,
                                )?;
                            }
                            probe!(default, slow_path_trace, 1);
                        }
                        0b011 => {
                            let cycles = self.machine.inner_mut().imc.traces[slot].cycles;
                            self.machine.add_cycles(cycles)?;
                            for instruction in
                                self.machine.inner_mut().imc.traces[slot].instructions
                            {
                                if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                                    break;
                                }
                                if is_slowpath_instruction(instruction) {
                                    let cycles = self.machine.instruction_cycle_func()(
                                        instruction,
                                        self.machine.inner.rvv.vl(),
                                        self.machine.inner.rvv.vsew(),
                                    );
                                    self.machine.add_cycles(cycles)?;
                                }
                                execute_nocheck(
                                    &mut self.machine,
                                    &handle_function_list,
                                    instruction,
                                )?;
                            }
                            probe!(default, slow_path_trace, 1);
                        }
                        _ => {
                            match vtrace_type(
                                &self.machine.inner_mut().imc.traces[slot].instructions,
                            ) {
                                VTraceType::StableCheckAndCycles => {
                                    let mut cycles =
                                        self.machine.inner_mut().imc.traces[slot].cycles;
                                    self.machine.add_cycles(cycles)?;
                                    for instruction in
                                        self.machine.inner_mut().imc.traces[slot].instructions
                                    {
                                        if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                                            break;
                                        }
                                        if is_slowpath_instruction(instruction) {
                                            let comming_cycles =
                                                self.machine.instruction_cycle_func()(
                                                    instruction,
                                                    self.machine.inner.rvv.vl(),
                                                    self.machine.inner.rvv.vsew(),
                                                );
                                            self.machine.add_cycles(comming_cycles)?;
                                            cycles += comming_cycles;
                                        }
                                        execute(
                                            &mut self.machine,
                                            &vcheck_function_list,
                                            &handle_function_list,
                                            instruction,
                                        )?;
                                    }
                                    self.machine.inner_mut().imc.traces[slot].slowpath = 0b101;
                                    self.machine.inner_mut().imc.traces[slot].cycles = cycles;
                                }
                                VTraceType::StableCheck => {
                                    let cycles = self.machine.inner_mut().imc.traces[slot].cycles;
                                    self.machine.add_cycles(cycles)?;
                                    for instruction in
                                        self.machine.inner_mut().imc.traces[slot].instructions
                                    {
                                        if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                                            break;
                                        }
                                        if is_slowpath_instruction(instruction) {
                                            let cycles = self.machine.instruction_cycle_func()(
                                                instruction,
                                                self.machine.inner.rvv.vl(),
                                                self.machine.inner.rvv.vsew(),
                                            );
                                            self.machine.add_cycles(cycles)?;
                                        }
                                        execute(
                                            &mut self.machine,
                                            &vcheck_function_list,
                                            &handle_function_list,
                                            instruction,
                                        )?;
                                    }
                                    self.machine.inner_mut().imc.traces[slot].slowpath = 0b011;
                                }
                                VTraceType::Unstable => {
                                    let cycles = self.machine.inner_mut().imc.traces[slot].cycles;
                                    self.machine.add_cycles(cycles)?;
                                    for instruction in
                                        self.machine.inner_mut().imc.traces[slot].instructions
                                    {
                                        if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                                            break;
                                        }
                                        if is_slowpath_instruction(instruction) {
                                            let cycles = self.machine.instruction_cycle_func()(
                                                instruction,
                                                self.machine.inner.rvv.vl(),
                                                self.machine.inner.rvv.vsew(),
                                            );
                                            self.machine.add_cycles(cycles)?;
                                        }
                                        execute(
                                            &mut self.machine,
                                            &vcheck_function_list,
                                            &handle_function_list,
                                            instruction,
                                        )?;
                                    }
                                }
                            }
                            probe!(default, slow_path_trace, 1);
                        }
                    }
                }
                _ => return Err(Error::Asm(result)),
            }
        }
        Ok(self.machine.exit_code())
    }

    pub fn step(&mut self, decoder: &mut Decoder) -> Result<(), Error> {
        let vcheck_function_list =
            generate_vcheck_function_list::<DefaultMachine<AsmGlueMachine>>();
        let handle_function_list =
            generate_handle_function_list::<DefaultMachine<AsmGlueMachine>>();
        // Decode only one instruction into a trace
        let pc = *self.machine.pc();
        let slot = calculate_slot(pc);
        let mut trace = Trace::default();
        let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
        let len = instruction_length(instruction) as u8;
        trace.instructions[0] = instruction;
        let vl = self.machine.inner.rvv.vl();
        let sew = self.machine.inner.rvv.vsew();
        trace.cycles += self.machine.instruction_cycle_func()(instruction, vl, sew);
        let opcode = extract_opcode(instruction);
        trace.thread[0] = unsafe {
            u64::from(*(ckb_vm_asm_labels as *const u32).offset(opcode as isize))
                + (ckb_vm_asm_labels as *const u32 as u64)
        };
        trace.instructions[1] = blank_instruction(OP_CUSTOM_TRACE_END);
        trace.thread[1] = unsafe {
            u64::from(*(ckb_vm_asm_labels as *const u32).offset(OP_CUSTOM_TRACE_END as isize))
                + (ckb_vm_asm_labels as *const u32 as u64)
        };
        trace.address = pc;
        trace.length = len;
        self.machine.inner_mut().imc.traces[slot] = trace;

        let result = unsafe { ckb_vm_x64_execute(&mut (*self.machine.inner_mut().imc)) };
        match result {
            RET_DECODE_TRACE => (),
            RET_ECALL => self.machine.ecall()?,
            RET_EBREAK => self.machine.ebreak()?,
            RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
            RET_OUT_OF_BOUND => return Err(Error::MemOutOfBound),
            RET_INVALID_PERMISSION => return Err(Error::MemWriteOnExecutablePage),
            RET_SLOWPATH => {
                let pc = *self.machine.pc() - 4;
                let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
                execute_instruction(
                    &mut self.machine,
                    &vcheck_function_list,
                    &handle_function_list,
                    instruction,
                )?;
            }
            RET_SLOWPATH_TRACE => {
                let pc = *self.machine.pc();
                let slot = calculate_slot(pc);
                let cycles = self.machine.inner_mut().imc.traces[slot].cycles;
                self.machine.add_cycles(cycles)?;
                for instruction in self.machine.inner_mut().imc.traces[slot].instructions {
                    if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                        break;
                    }
                    execute(
                        &mut self.machine,
                        &vcheck_function_list,
                        &handle_function_list,
                        instruction,
                    )?;
                }
            }
            _ => return Err(Error::Asm(result)),
        }
        self.machine.inner_mut().imc.traces[slot] = Trace::default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asm_constant_rules() {
        assert!(TRACE_ITEM_LENGTH * 4 < 256);
    }
}
