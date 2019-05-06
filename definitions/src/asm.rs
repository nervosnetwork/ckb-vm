use crate::{instructions::Instruction, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY};
use std::alloc::{alloc_zeroed, Layout};
use std::cmp::min;

// The number of trace items to keep
pub const TRACE_SIZE: usize = 8192;
pub const TRACE_ITEM_LENGTH: usize = 16;

pub const RET_DECODE_TRACE: u8 = 1;
pub const RET_ECALL: u8 = 2;
pub const RET_EBREAK: u8 = 3;
pub const RET_MAX_CYCLES_EXCEEDED: u8 = 4;
pub const RET_OUT_OF_BOUND: u8 = 5;

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
    pub elf_end: usize,
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
            #[allow(clippy::cast_ptr_alignment)]
            // TODO: change this to alloc so we are using malloc instead of
            // calloc, then do lazy zero filling when necessary. That might
            // save us some time in case a script doesn't use all the memory.
            // Right now this calloc phase here takes around 1ms.
            let raw_allocation = alloc_zeroed(layout) as *mut AsmCoreMachine;
            Box::from_raw(raw_allocation)
        };
        machine.max_cycles = max_cycles;
        machine
    }

    pub fn clear_traces(&mut self, address: u64, length: u64) {
        let end = address + length;
        let minimal_slot = calculate_slot(address.saturating_sub(TRACE_ITEM_LENGTH as u64 * 4));
        let maximal_slot = calculate_slot(end);
        for slot in minimal_slot..=min(maximal_slot, self.traces.len()) {
            let slot_address = self.traces[slot].address;
            let slot_end = slot_address + u64::from(self.traces[slot].length);
            if !((end <= slot_address) || (slot_end <= address)) {
                self.traces[slot] = Trace::default();
            }
        }
    }
}
