use crate::{
    instructions::Instruction, MEMORY_FRAMES, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY,
    RISCV_PAGES,
};
use std::alloc::{alloc, Layout};

// The number of trace items to keep
pub const TRACE_SIZE: usize = 8192;
pub const TRACE_ITEM_LENGTH: usize = 16;

pub const RET_DECODE_TRACE: u8 = 1;
pub const RET_ECALL: u8 = 2;
pub const RET_EBREAK: u8 = 3;
pub const RET_DYNAMIC_JUMP: u8 = 4;
pub const RET_MAX_CYCLES_EXCEEDED: u8 = 5;
pub const RET_OUT_OF_BOUND: u8 = 6;
pub const RET_INVALID_PERMISSION: u8 = 7;

#[inline(always)]
pub fn calculate_slot(addr: u64) -> usize {
    (addr as usize >> 5) & (TRACE_SIZE - 1)
}

#[derive(Default)]
#[repr(C)]
pub struct Trace {
    pub address: u64,
    pub length: u8,
    pub cycles: u64,
    pub instructions: [Instruction; TRACE_ITEM_LENGTH + 1],
    // We are using direct threaded code here:
    // https://en.wikipedia.org/wiki/Threaded_code
    pub thread: [u64; TRACE_ITEM_LENGTH + 1],
}

#[repr(C)]
pub struct AsmCoreMachine {
    pub registers: [u64; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: u64,
    pub running: u8,
    pub cycles: u64,
    pub max_cycles: u64,
    pub version: u32,
    pub flags: [u8; RISCV_PAGES],
    pub memory: [u8; RISCV_MAX_MEMORY],
    pub frames: [u8; MEMORY_FRAMES],
    pub traces: [Trace; TRACE_SIZE],
}

impl Default for Box<AsmCoreMachine> {
    fn default() -> Self {
        // Since in AsmCoreMachine we will always have max_cycles, the best
        // way to solve the case that a max cycle is not available, is just
        // to assign the maximum value allowed in u64.
        AsmCoreMachine::new_with_max_cycles(u64::max_value())
    }
}

impl AsmCoreMachine {
    pub fn new(version: u32, max_cycles: u64) -> Box<AsmCoreMachine> {
        let mut machine = Self::new_with_max_cycles(max_cycles);
        machine.version = version;
        machine
    }

    pub fn new_with_max_cycles(max_cycles: u64) -> Box<AsmCoreMachine> {
        let mut machine = unsafe {
            let layout = Layout::new::<AsmCoreMachine>();
            #[allow(clippy::cast_ptr_alignment)]
            // Use alloc so we are using malloc instead of
            // calloc, then do lazy zero filling when necessary. That might
            // save us some time in case a script doesn't use all the memory.
            let raw_allocation = alloc(layout) as *mut AsmCoreMachine;
            Box::from_raw(raw_allocation)
        };
        machine.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
        machine.pc = 0;
        machine.running = 0;
        machine.cycles = 0;
        machine.max_cycles = max_cycles;
        machine.flags = [0; RISCV_PAGES];
        for i in 0..TRACE_SIZE {
            machine.traces[i] = Trace::default();
        }
        machine.frames = [0; MEMORY_FRAMES];
        machine
    }
}
