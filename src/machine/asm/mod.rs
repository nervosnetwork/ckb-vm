use std::mem::transmute;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use ckb_vm_definitions::{
    asm::{
        calculate_slot, Trace, RET_DECODE_TRACE, RET_DYNAMIC_JUMP, RET_EBREAK, RET_ECALL,
        RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND, RET_SLOWPATH,
        TRACE_ITEM_LENGTH,
    },
    instructions::OP_CUSTOM_TRACE_END,
    MEMORY_FRAME_PAGE_SHIFTS,
};
use goblin::elf::Elf;
use libc::c_uchar;
use rand::{prelude::RngCore, SeedableRng};

use crate::{
    decoder::{build_decoder, Decoder},
    instructions::{
        blank_instruction, execute_instruction, extract_opcode, instruction_length,
        is_basic_block_end_instruction,
    },
    machine::aot::AotCode,
    memory::{
        check_permission, fill_page_data, get_page_indices, memset, round_page_down, round_page_up,
        set_dirty, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE,
    },
    CoreMachine, DefaultMachine, Error, Machine, Memory, SupportMachine, MEMORY_FRAME_SHIFTS,
    RISCV_MAX_MEMORY, RISCV_PAGES, RISCV_PAGESIZE,
};

pub use ckb_vm_definitions::asm::AsmCoreMachine;

impl CoreMachine for Box<AsmCoreMachine> {
    type REG = u64;
    type MEM = Self;

    fn pc(&self) -> &Self::REG {
        &self.pc
    }

    fn set_pc(&mut self, next_pc: Self::REG) {
        self.pc = next_pc
    }

    fn memory(&self) -> &Self {
        &self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
    }

    fn registers(&self) -> &[Self::REG] {
        &self.registers
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.registers[idx] = value;
    }

    fn isa(&self) -> u8 {
        self.isa
    }

    fn version(&self) -> u32 {
        self.version
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

fn check_memory(machine: &mut AsmCoreMachine, page_indices: &(u64, u64)) -> Result<(), Error> {
    let frame = page_indices.0 >> MEMORY_FRAME_PAGE_SHIFTS;
    let frame_end = page_indices.1 >> MEMORY_FRAME_PAGE_SHIFTS;
    for i in frame..=frame_end {
        if machine.frames[i as usize] == 0 {
            inited_memory(i, machine);
            machine.frames[i as usize] = 1;
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
            return Err(Error::Unaligned);
        }
        if addr > RISCV_MAX_MEMORY as u64
            || size > RISCV_MAX_MEMORY as u64
            || addr + size > RISCV_MAX_MEMORY as u64
            || offset_from_addr > size
        {
            return Err(Error::OutOfBound);
        }
        // We benchmarked the code piece here, using while loop this way is
        // actually faster than a for..in solution. The difference is roughly
        // 3% so we are keeping this version.
        let mut current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            if self.fetch_flag(page)? & FLAG_FREEZED != 0 {
                return Err(Error::InvalidPermission);
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
            Err(Error::OutOfBound)
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < RISCV_PAGES as u64 {
            self.flags[page as usize] |= flag;
            Ok(())
        } else {
            Err(Error::OutOfBound)
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < RISCV_PAGES as u64 {
            self.flags[page as usize] &= !flag;
            Ok(())
        } else {
            Err(Error::OutOfBound)
        }
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, value.len() as u64)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        check_memory(self, &page_indices)?;
        set_dirty(self, &page_indices)?;
        let slice = &mut self.memory[addr as usize..addr as usize + value.len()];
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, size)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        check_memory(self, &page_indices)?;
        set_dirty(self, &page_indices)?;
        memset(
            &mut self.memory[addr as usize..(addr + size) as usize],
            value,
        );
        Ok(())
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        let page_indices = get_page_indices(addr, 2)?;
        check_permission(self, &page_indices, FLAG_EXECUTABLE)?;
        self.load16(&(addr)).map(|v| v as u16)
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        let page_indices = get_page_indices(addr, 4)?;
        check_permission(self, &page_indices, FLAG_EXECUTABLE)?;
        self.load32(&(addr)).map(|v| v as u32)
    }

    fn load8(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 1)?;
        check_memory(self, &page_indices)?;
        Ok(u64::from(self.memory[addr as usize]))
    }

    fn load16(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 2)?;
        check_memory(self, &page_indices)?;
        Ok(u64::from(LittleEndian::read_u16(
            &self.memory[addr as usize..addr as usize + 2],
        )))
    }

    fn load32(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 4)?;
        check_memory(self, &page_indices)?;
        Ok(u64::from(LittleEndian::read_u32(
            &self.memory[addr as usize..addr as usize + 4],
        )))
    }

    fn load64(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 8)?;
        check_memory(self, &page_indices)?;
        Ok(LittleEndian::read_u64(
            &self.memory[addr as usize..addr as usize + 8],
        ))
    }

    fn store8(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 1)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        check_memory(self, &page_indices)?;
        set_dirty(self, &page_indices)?;
        self.memory[addr as usize] = (*value) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 2)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        check_memory(self, &page_indices)?;
        set_dirty(self, &page_indices)?;
        LittleEndian::write_u16(
            &mut self.memory[addr as usize..(addr + 2) as usize],
            *value as u16,
        );
        Ok(())
    }

    fn store32(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 4)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        check_memory(self, &page_indices)?;
        set_dirty(self, &page_indices)?;
        LittleEndian::write_u32(
            &mut self.memory[addr as usize..(addr + 4) as usize],
            *value as u32,
        );
        Ok(())
    }

    fn store64(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        let page_indices = get_page_indices(addr, 8)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        check_memory(self, &page_indices)?;
        set_dirty(self, &page_indices)?;
        LittleEndian::write_u64(&mut self.memory[addr as usize..(addr + 8) as usize], *value);
        Ok(())
    }
}

impl SupportMachine for Box<AsmCoreMachine> {
    fn cycles(&self) -> u64 {
        self.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.cycles = cycles;
    }

    fn max_cycles(&self) -> Option<u64> {
        Some(self.max_cycles)
    }

    fn running(&self) -> bool {
        self.running == 1
    }

    fn set_running(&mut self, running: bool) {
        self.running = if running { 1 } else { 0 }
    }
}

#[derive(Default)]
pub struct AsmMachine<'a> {
    pub machine: DefaultMachine<'a, Box<AsmCoreMachine>>,
    pub aot_code: Option<&'a AotCode>,
}

extern "C" {
    fn ckb_vm_x64_execute(m: *mut AsmCoreMachine) -> c_uchar;
    // We are keeping this as a function here, but at the bottom level this really
    // just points to an array of assembly label offsets for each opcode.
    fn ckb_vm_asm_labels();
}

impl<'a> AsmMachine<'a> {
    pub fn new(
        machine: DefaultMachine<'a, Box<AsmCoreMachine>>,
        aot_code: Option<&'a AotCode>,
    ) -> Self {
        Self { machine, aot_code }
    }

    pub fn default_with_aot_code(aot_code: &'a AotCode) -> Self {
        Self {
            machine: DefaultMachine::<'a, Box<AsmCoreMachine>>::default(),
            aot_code: Some(aot_code),
        }
    }

    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }

    pub fn load_program_elf(
        &mut self,
        program: &Bytes,
        args: &[Bytes],
        elf: &Elf,
    ) -> Result<u64, Error> {
        self.machine.load_program_elf(program, args, elf)
    }

    pub fn run(&mut self) -> Result<i8, Error> {
        let decoder = build_decoder::<u64>(self.machine.isa(), self.machine.version());
        self.machine.set_running(true);
        while self.machine.running() {
            let result = if let Some(aot_code) = &self.aot_code {
                if let Some(offset) = aot_code.labels.get(self.machine.pc()) {
                    let base_address = aot_code.base_address();
                    let offset_address = base_address + u64::from(*offset);
                    let f = unsafe {
                        transmute::<u64, fn(*mut AsmCoreMachine, u64) -> u8>(base_address)
                    };
                    f(&mut (**self.machine.inner_mut()), offset_address)
                } else {
                    unsafe { ckb_vm_x64_execute(&mut (**self.machine.inner_mut())) }
                }
            } else {
                unsafe { ckb_vm_x64_execute(&mut (**self.machine.inner_mut())) }
            };
            match result {
                RET_DECODE_TRACE => {
                    let pc = *self.machine.pc();
                    let slot = calculate_slot(pc);
                    let mut trace = Trace::default();
                    let mut current_pc = pc;
                    let mut i = 0;
                    while i < TRACE_ITEM_LENGTH {
                        let mut instruction =
                            decoder.decode(self.machine.memory_mut(), current_pc)?;
                        let end_instruction = is_basic_block_end_instruction(instruction);
                        current_pc += u64::from(instruction_length(instruction));
                        // We are storing the offset after current instruction in unused
                        // space of the instruction, so as to allow easy access of this data
                        // within assembly loops.
                        instruction |= u64::from((current_pc - pc) as u8) << 24;
                        trace.instructions[i] = instruction;
                        trace.cycles += self
                            .machine
                            .instruction_cycle_func()
                            .as_ref()
                            .map(|f| f(instruction))
                            .unwrap_or(0);
                        let opcode = extract_opcode(instruction);
                        // Here we are calculating the absolute address used in direct threading
                        // from label offsets.
                        trace.thread[i] = unsafe {
                            u64::from(*(ckb_vm_asm_labels as *const u32).offset(opcode as isize))
                                + (ckb_vm_asm_labels as *const u32 as u64)
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
                    self.machine.inner_mut().traces[slot] = trace;
                }
                RET_ECALL => self.machine.ecall()?,
                RET_EBREAK => self.machine.ebreak()?,
                RET_DYNAMIC_JUMP => (),
                RET_MAX_CYCLES_EXCEEDED => return Err(Error::InvalidCycles),
                RET_OUT_OF_BOUND => return Err(Error::OutOfBound),
                RET_INVALID_PERMISSION => return Err(Error::InvalidPermission),
                RET_SLOWPATH => {
                    let pc = *self.machine.pc() - 4;
                    let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
                    execute_instruction(instruction, &mut self.machine)?;
                }
                _ => return Err(Error::Asm(result)),
            }
        }
        Ok(self.machine.exit_code())
    }

    pub fn step(&mut self, decoder: &Decoder) -> Result<(), Error> {
        // Decode only one instruction into a trace
        let pc = *self.machine.pc();
        let slot = calculate_slot(pc);
        let mut trace = Trace::default();
        let mut instruction = decoder.decode(self.machine.memory_mut(), pc)?;
        let len = instruction_length(instruction) as u8;
        instruction |= u64::from(len) << 24;
        trace.instructions[0] = instruction;
        trace.cycles += self
            .machine
            .instruction_cycle_func()
            .as_ref()
            .map(|f| f(instruction))
            .unwrap_or(0);
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
        self.machine.inner_mut().traces[slot] = trace;

        let result = unsafe { ckb_vm_x64_execute(&mut (**self.machine.inner_mut())) };
        match result {
            RET_DECODE_TRACE => (),
            RET_ECALL => self.machine.ecall()?,
            RET_EBREAK => self.machine.ebreak()?,
            RET_MAX_CYCLES_EXCEEDED => return Err(Error::InvalidCycles),
            RET_OUT_OF_BOUND => return Err(Error::OutOfBound),
            RET_INVALID_PERMISSION => return Err(Error::InvalidPermission),
            RET_SLOWPATH => {
                let pc = *self.machine.pc() - 4;
                let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
                execute_instruction(instruction, &mut self.machine)?;
            }
            _ => return Err(Error::Asm(result)),
        }
        self.machine.inner_mut().traces[slot] = Trace::default();
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
