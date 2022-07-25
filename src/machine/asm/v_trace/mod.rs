pub mod infer;

use crate::{
    decoder::build_decoder,
    instructions::{
        blank_instruction, common, execute, extract_opcode, instruction_length,
        is_basic_block_end_instruction, is_slowpath_instruction, v_alu as alu, Instruction, Itype,
        Register, Rtype, Stype, VVtype, VXtype,
    },
    machine::{
        asm::{
            ckb_vm_asm_labels, ckb_vm_x64_execute, v_trace::infer::VInferMachine, AotCode,
            AsmCoreMachine, Error,
        },
        VERSION0,
    },
    CoreMachine, DefaultMachine, Machine, SupportMachine,
};
use bytes::Bytes;
use ckb_vm_definitions::{
    asm::{
        calculate_slot, Trace, RET_CYCLES_OVERFLOW, RET_DECODE_TRACE, RET_DYNAMIC_JUMP, RET_EBREAK,
        RET_ECALL, RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND,
        RET_SLOWPATH_TRACE, TRACE_ITEM_LENGTH,
    },
    instructions::{self as insts, OP_CUSTOM_TRACE_END},
    ISA_MOP,
};
use eint::{Eint, E256, E512};
use std::collections::HashMap;
use std::mem::transmute;

pub const VTRACE_MAX_LENGTH: usize = 32;

pub struct VTrace<M: Machine> {
    pub address: u64,
    pub code_length: usize,
    pub sizes: Vec<u8>,
    pub actions: Vec<Box<dyn Fn(&mut M) -> Result<(), Error>>>,
}

impl<M: Machine> Default for VTrace<M> {
    fn default() -> Self {
        VTrace {
            address: 0,
            code_length: 0,
            sizes: Vec::new(),
            actions: Vec::new(),
        }
    }
}

type CM<'a> = DefaultMachine<'a, Box<AsmCoreMachine>>;

pub struct VTraceAsmMachine<'a> {
    pub machine: CM<'a>,
    pub aot_code: Option<&'a AotCode>,
    pub v_traces: HashMap<u64, VTrace<CM<'a>>>,
}

impl<'a> VTraceAsmMachine<'a> {
    pub fn new(machine: CM<'a>, aot_code: Option<&'a AotCode>) -> Self {
        let mut r = Self {
            machine,
            aot_code,
            v_traces: HashMap::default(),
        };
        // Default to illegal configuration
        r.machine.set_vl(0, 0, 0, u64::MAX);
        r
    }

    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.machine.inner.max_cycles = cycles;
    }

    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }

    pub fn run(&mut self) -> Result<i8, Error> {
        if self.machine.isa() & ISA_MOP != 0 && self.machine.version() == VERSION0 {
            return Err(Error::InvalidVersion);
        }
        let mut decoder = build_decoder::<u64>(self.machine.isa(), self.machine.version());
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
                        let instruction = decoder.decode(self.machine.memory_mut(), current_pc)?;
                        let end_instruction = is_basic_block_end_instruction(instruction);
                        current_pc += u64::from(instruction_length(instruction));
                        if trace.slowpath == 0 && is_slowpath_instruction(instruction) {
                            trace.slowpath = 1;
                        }
                        trace.instructions[i] = instruction;
                        // don't count cycles in trace for RVV instructions. They
                        // will be counted in slow path.
                        trace.cycles += self
                            .machine
                            .instruction_cycle_func()
                            .as_ref()
                            .map(|f| f(instruction, 0, 0, true))
                            .unwrap_or(0);
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

                    if trace.slowpath != 0 && self.v_traces.get(&pc).is_none() {
                        if let Some(v_trace) = Self::try_build_v_trace(&trace) {
                            self.v_traces.insert(pc, v_trace);
                        }
                    }
                    self.machine.inner_mut().traces[slot] = trace;
                }
                RET_ECALL => self.machine.ecall()?,
                RET_EBREAK => self.machine.ebreak()?,
                RET_DYNAMIC_JUMP => (),
                RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
                RET_CYCLES_OVERFLOW => return Err(Error::CyclesOverflow),
                RET_OUT_OF_BOUND => return Err(Error::MemOutOfBound),
                RET_INVALID_PERMISSION => return Err(Error::MemWriteOnExecutablePage),
                RET_SLOWPATH_TRACE => loop {
                    let pc = *self.machine.pc();
                    let slot = calculate_slot(pc);
                    let slowpath = self.machine.inner_mut().traces[slot].slowpath;

                    if slowpath == 0 {
                        break;
                    }
                    let cycles = self.machine.inner_mut().traces[slot].cycles;
                    self.machine.add_cycles(cycles)?;

                    if let Some(v_trace) = self.v_traces.get(&pc) {
                        // Optimized VTrace
                        // println!("Running trace: {:x}, length: {}", v_trace.address, v_trace.actions.len());
                        for (i, action) in v_trace.actions.iter().enumerate() {
                            let instruction_size = v_trace.sizes[i];
                            let next_pc = self
                                .machine
                                .pc()
                                .overflowing_add(&(instruction_size as u64));
                            self.machine.update_pc(next_pc);
                            action(&mut self.machine)?;
                            self.machine.commit_pc();
                        }
                    } else {
                        // VTrace is not avaiable, fallback to plain executing mode
                        for instruction in self.machine.inner_mut().traces[slot].instructions {
                            if instruction == blank_instruction(OP_CUSTOM_TRACE_END) {
                                break;
                            }
                            execute(instruction, &mut self.machine)?;
                        }
                    }
                },
                _ => return Err(Error::Asm(result)),
            }
        }
        println!("Total v traces: {}", self.v_traces.len());
        Ok(self.machine.exit_code())
    }

    pub fn try_build_v_trace(trace: &Trace) -> Option<VTrace<CM<'a>>> {
        // TODO: run the trace first on a dummy machine to *typecheck*
        // V instructions.
        let mut v_trace = VTrace {
            address: trace.address,
            code_length: trace.length as usize,
            ..Default::default()
        };

        let mut i = 0;
        let mut first_v_processed = false;
        let mut infer_machine = VInferMachine::default();
        while extract_opcode(trace.instructions[i]) != OP_CUSTOM_TRACE_END {
            let inst = trace.instructions[i];
            i += 1;
            v_trace.sizes.push(instruction_length(inst));

            let opcode = extract_opcode(inst);
            if !is_slowpath_instruction(inst) {
                match opcode {
                    insts::OP_ADD => {
                        v_trace.actions.push(Box::new(move |m| handle_add(m, inst)));
                    }
                    insts::OP_ADDI => {
                        v_trace
                            .actions
                            .push(Box::new(move |m| handle_addi(m, inst)));
                    }
                    insts::OP_SUB => {
                        v_trace.actions.push(Box::new(move |m| handle_sub(m, inst)));
                    }
                    insts::OP_SLLI => {
                        v_trace
                            .actions
                            .push(Box::new(move |m| handle_slli(m, inst)));
                    }
                    insts::OP_BLT => {
                        v_trace.actions.push(Box::new(move |m| handle_blt(m, inst)));
                    }
                    _ => panic!(
                        "Unexpected IMC instruction: {}",
                        insts::instruction_opcode_name(opcode)
                    ),
                    // _ => return None,
                }
                continue;
            }
            if !first_v_processed {
                // The first V instruction must be vsetvli
                // so as to guard against vl/vtype values.
                if ![insts::OP_VSETVLI].contains(&opcode) {
                    return None;
                }
                first_v_processed = true;
            }
            match opcode {
                insts::OP_VSETVLI => {
                    execute(inst, &mut infer_machine).ok()?;
                    v_trace
                        .actions
                        .push(Box::new(move |m: &mut CM<'a>| handle_vsetvli(m, inst)));
                }
                insts::OP_VLSE256_V => {
                    execute(inst, &mut infer_machine).ok()?;
                    v_trace
                        .actions
                        .push(Box::new(move |m: &mut CM<'a>| handle_vlse256(m, inst)));
                }
                insts::OP_VLE256_V => {
                    execute(inst, &mut infer_machine).ok()?;
                    v_trace
                        .actions
                        .push(Box::new(move |m: &mut CM<'a>| handle_vle256(m, inst)));
                }
                insts::OP_VSE256_V => {
                    execute(inst, &mut infer_machine).ok()?;
                    v_trace
                        .actions
                        .push(Box::new(move |m: &mut CM<'a>| handle_vse256(m, inst)));
                }
                insts::OP_VADD_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vadd_256(m, inst)));
                        }
                        512 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vadd_512(m, inst)));
                        }
                        _ => panic!("Unsupported vwadc.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VMADC_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vmadc_256(m, inst)));
                        }
                        512 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vmadc_512(m, inst)));
                        }
                        _ => panic!("Unsupported vmadc.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VSUB_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vsub_256(m, inst)));
                        }
                        _ => panic!("Unsupported vsub.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VMSBC_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vmsbc_256(m, inst)));
                        }
                        _ => panic!("Unsupported vmsbc.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VWMULU_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vmmulu_256(m, inst)));
                        }
                        _ => panic!("Unsupported vwmulu.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VMUL_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vmul_256(m, inst)));
                        }
                        _ => panic!("Unsupported vmulu.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VXOR_VV => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vxor_256(m, inst)));
                        }
                        _ => panic!("Unsupported vxor.vv with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VNSRL_WX => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vnsrl_256(m, inst)));
                        }
                        _ => panic!("Unsupported vnsrl.wx with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                insts::OP_VMANDNOT_MM => {
                    execute(inst, &mut infer_machine).ok()?;
                    v_trace
                        .actions
                        .push(Box::new(move |m: &mut CM<'a>| handle_vmandnot(m, inst)));
                }
                insts::OP_VMXOR_MM => {
                    execute(inst, &mut infer_machine).ok()?;
                    v_trace
                        .actions
                        .push(Box::new(move |m: &mut CM<'a>| handle_vmxor(m, inst)));
                }
                insts::OP_VMERGE_VVM => {
                    let sew = infer_machine.vsew();
                    execute(inst, &mut infer_machine).ok()?;
                    match sew {
                        256 => {
                            v_trace
                                .actions
                                .push(Box::new(move |m: &mut CM<'a>| handle_vmerge_256(m, inst)));
                        }
                        _ => panic!("Unsupported vmerge.vvm with sew: {}", infer_machine.vsew()),
                        // _ => return None,
                    }
                }
                _ => panic!(
                    "Unsupported v op: {}, sew now: {}",
                    insts::instruction_opcode_name(opcode),
                    infer_machine.vsew(),
                ),
                // _ => return None,
            };
        }
        println!(
            "Built trace: {:x}, length: {}",
            v_trace.address,
            v_trace.actions.len()
        );
        Some(v_trace)
    }
}

fn handle_vsetvli<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::set_vl(
        m,
        i.rd(),
        i.rs1(),
        m.registers()[i.rs1()].to_u64(),
        i.immediate_u() as u64,
    )
}

fn handle_vlse256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VXtype(inst);
    let addr = m.registers()[i.rs1()].to_u64();
    let stride = m.registers()[i.vs2()].to_u64();

    for j in 0..m.vl() {
        if i.vm() == 0 && !m.get_bit(0, j as usize) {
            continue;
        }

        m.mem_to_v(
            i.vd(),
            32 << 3,
            j as usize,
            stride.wrapping_mul(j).wrapping_add(addr),
        )?;
    }
    Ok(())
}

fn handle_vle256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VXtype(inst);
    let addr = m.registers()[i.rs1()].to_u64();
    let stride = 32u64;

    for j in 0..m.vl() {
        if i.vm() == 0 && !m.get_bit(0, j as usize) {
            continue;
        }
        m.mem_to_v(
            i.vd(),
            32 << 3,
            j as usize,
            stride.wrapping_mul(j).wrapping_add(addr),
        )?;
    }
    Ok(())
}

fn handle_vse256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VXtype(inst);
    let addr = m.registers()[i.rs1()].to_u64();
    let stride = 32u64;

    for j in 0..m.vl() {
        if i.vm() == 0 && !m.get_bit(0, j as usize) {
            continue;
        }
        m.v_to_mem(
            i.vd(),
            32 << 3,
            j as usize,
            stride.wrapping_mul(j).wrapping_add(addr),
        )?;
    }
    Ok(())
}

fn handle_vadd_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E256::get(m.element_ref(i.vs2(), 256, j));
        let a = E256::get(m.element_ref(i.vs1(), 256, j));
        let r = Eint::wrapping_add(b, a);
        r.put(m.element_mut(i.vd(), 256, j));
    }
    Ok(())
}

fn handle_vadd_512<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E512::get(m.element_ref(i.vs2(), 512, j));
        let a = E512::get(m.element_ref(i.vs1(), 512, j));
        let r = Eint::wrapping_add(b, a);
        r.put(m.element_mut(i.vd(), 512, j));
    }
    Ok(())
}

fn handle_vmadc_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E256::get(m.element_ref(i.vs2(), sew, j));
        let a = E256::get(m.element_ref(i.vs1(), sew, j));
        if alu::madc(b, a) {
            m.set_bit(i.vd(), j);
        } else {
            m.clr_bit(i.vd(), j);
        };
    }
    Ok(())
}

fn handle_vmadc_512<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 512;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E512::get(m.element_ref(i.vs2(), sew, j));
        let a = E512::get(m.element_ref(i.vs1(), sew, j));
        if alu::madc(b, a) {
            m.set_bit(i.vd(), j);
        } else {
            m.clr_bit(i.vd(), j);
        };
    }
    Ok(())
}

fn handle_vsub_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E256::get(m.element_ref(i.vs2(), sew, j));
        let a = E256::get(m.element_ref(i.vs1(), sew, j));
        let r = Eint::wrapping_sub(b, a);
        r.put(m.element_mut(i.vd(), sew, j));
    }
    Ok(())
}

fn handle_vmsbc_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E256::get(m.element_ref(i.vs2(), sew, j));
        let a = E256::get(m.element_ref(i.vs1(), sew, j));
        if alu::msbc(b, a) {
            m.set_bit(i.vd(), j);
        } else {
            m.clr_bit(i.vd(), j);
        };
    }
    Ok(())
}

use std::arch::asm;

#[inline(never)]
pub fn widening_mul_256(a: &[u8], b: &[u8], dst: &mut [u8], len: usize) {
    for i in 0..len {
        // Inspired from https://github.com/cloudflare/bn256/blob/9bd9f73a0273ed2f42707ed13b3e36d38baa2a49/mul_amd64.h#L1
        unsafe {
            asm!(
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "mov [rdi + 0], r8",
                "mov [rdi + 8], r9",
                "mov [rdi + 16], r10",
                "mov [rdi + 24], r11",
                "mov [rdi + 32], r12",
                "",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "add r8, [rdi + 8]",
                "adc r9, [rdi + 16]",
                "adc r10, [rdi + 24]",
                "adc r11, [rdi + 32]",
                "adc r12, 0",
                "mov [rdi + 8], r8",
                "mov [rdi + 16], r9",
                "mov [rdi + 24], r10",
                "mov [rdi + 32], r11",
                "mov [rdi + 40], r12",
                "",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "add r8, [rdi + 16]",
                "adc r9, [rdi + 24]",
                "adc r10, [rdi + 32]",
                "adc r11, [rdi + 40]",
                "adc r12, 0",
                "mov [rdi + 16], r8",
                "mov [rdi + 24], r9",
                "mov [rdi + 32], r10",
                "mov [rdi + 40], r11",
                "mov [rdi + 48], r12",
                "",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "add r8, [rdi + 24]",
                "adc r9, [rdi + 32]",
                "adc r10, [rdi + 40]",
                "adc r11, [rdi + 48]",
                "adc r12, 0",
                "mov [rdi + 24], r8",
                "mov [rdi + 32], r9",
                "mov [rdi + 40], r10",
                "mov [rdi + 48], r11",
                "mov [rdi + 56], r12",
                in("rsi") a.as_ptr() as usize + i * 32,
                in("rcx") b.as_ptr() as usize + i * 32,
                in("rdi") dst.as_ptr() as usize + i * 64,
                lateout("r12") _,
                clobber_abi("sysv64"),
                clobber_abi("win64"),
            );
        }
    }
}

fn handle_vmmulu_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j as usize) {
            continue;
        }
        // Shortcut to create [0u8; 64]
        let underlying_c = [0u64; 8];
        let mut c: [u8; 64] = unsafe { transmute(underlying_c) };

        widening_mul_256(
            m.element_ref(i.vs1(), sew, j),
            m.element_ref(i.vs2(), sew, j),
            &mut c,
            1,
        );

        m.element_mut(i.vd(), sew * 2, j).copy_from_slice(&c);
    }
    Ok(())
}

fn handle_vmul_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j as usize) {
            continue;
        }
        let b = E256::get(m.element_ref(i.vs2(), sew, j));
        let a = E256::get(m.element_ref(i.vs1(), sew, j));
        let r = Eint::wrapping_mul(b, a);
        r.put(m.element_mut(i.vd(), sew, j));
    }
    Ok(())
}

fn handle_vxor_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j as usize) {
            continue;
        }
        let b = E256::get(m.element_ref(i.vs2(), sew, j));
        let a = E256::get(m.element_ref(i.vs1(), sew, j));
        let r = alu::xor(b, a);
        r.put(m.element_mut(i.vd(), sew, j));
    }
    Ok(())
}

fn handle_vnsrl_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VXtype(inst);
    for j in 0..m.vl() as usize {
        if i.vm() == 0 && !m.get_bit(0, j) {
            continue;
        }
        let b = E512::get(m.element_ref(i.vs2(), sew * 2, j));
        let a = if 0 != 0 {
            E512::from(E256::from(m.registers()[i.rs1()].to_i64())).lo_sext()
        } else {
            E512::from(E256::from(m.registers()[i.rs1()].to_u64()))
        };
        let r = alu::srl(b, a);
        r.put_lo(m.element_mut(i.vd(), sew, j));
    }
    Ok(())
}

fn handle_vmandnot<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        let b = m.get_bit(i.vs2(), j);
        let a = m.get_bit(i.vs1(), j);
        if b & !a {
            m.set_bit(i.vd(), j);
        } else {
            m.clr_bit(i.vd(), j);
        }
    }
    Ok(())
}

fn handle_vmxor<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        let b = m.get_bit(i.vs2(), j);
        let a = m.get_bit(i.vs1(), j);
        if b ^ a {
            m.set_bit(i.vd(), j);
        } else {
            m.clr_bit(i.vd(), j);
        }
    }
    Ok(())
}

fn handle_vmerge_256<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let sew = 256;
    let i = VVtype(inst);
    for j in 0..m.vl() as usize {
        let mbit = m.get_bit(0, j);
        let b = E256::get(m.element_ref(i.vs2(), sew, j));
        let a = E256::get(m.element_ref(i.vs1(), sew, j));
        let r = alu::merge(b, a, mbit);
        r.put(m.element_mut(i.vd(), sew, j));
    }
    Ok(())
}

fn handle_sub<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::sub(m, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

fn handle_add<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::add(m, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

fn handle_addi<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::addi(m, i.rd(), i.rs1(), i.immediate_s());
    Ok(())
}

fn handle_slli<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::slli(m, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

fn handle_blt<'a>(m: &mut CM<'a>, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = m.pc();
    let rs1_value = &m.registers()[i.rs1()];
    let rs2_value = &m.registers()[i.rs2()];
    let condition = rs1_value.lt_s(&rs2_value);
    let new_pc = condition.cond(
        &u64::from_i32(i.immediate_s()).wrapping_add(*pc),
        &u64::from_u8(instruction_length(inst)).wrapping_add(*pc),
    );
    m.update_pc(new_pc);
    Ok(())
}
