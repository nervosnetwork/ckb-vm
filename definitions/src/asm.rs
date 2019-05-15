use crate::{
    instructions::Instruction, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGES,
};
use std::alloc::{alloc, Layout};
use std::ptr;

// The number of trace items to keep
pub const TRACE_SIZE: usize = 8192;
pub const TRACE_ITEM_LENGTH: usize = 16;

pub const RET_DECODE_TRACE: u8 = 1;
pub const RET_ECALL: u8 = 2;
pub const RET_EBREAK: u8 = 3;
pub const RET_MAX_CYCLES_EXCEEDED: u8 = 4;
pub const RET_OUT_OF_BOUND: u8 = 5;
pub const RET_INVALID_PERMISSION: u8 = 6;

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
    pub cycles: u64,
    pub max_cycles: u64,
    pub flags: [u8; RISCV_PAGES],
    pub memory: [u8; RISCV_MAX_MEMORY],
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
    pub fn new_with_max_cycles(max_cycles: u64) -> Box<AsmCoreMachine> {
        let mut machine = unsafe {
            let layout = Layout::new::<AsmCoreMachine>();
            let p = alloc(layout);
            #[allow(clippy::cast_ptr_alignment)]
            let m = Box::from_raw(p as *mut AsmCoreMachine);
            let memory_offset =
                (&m.memory as *const u8 as usize) - (&*m as *const AsmCoreMachine as usize);
            // This zeros all fields before memory field
            ptr::write_bytes(p, 0, memory_offset);
            m
        };
        machine.max_cycles = max_cycles;
        for trace in machine.traces.iter_mut() {
            trace.length = 0;
        }
        machine
    }
}
