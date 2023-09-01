use crate::{
    instructions::Instruction, MEMORY_FRAMESIZE, MEMORY_FRAME_SHIFTS,
    RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGESIZE,
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
pub const RET_CYCLES_OVERFLOW: u8 = 6;
pub const RET_OUT_OF_BOUND: u8 = 7;
pub const RET_INVALID_PERMISSION: u8 = 8;
pub const RET_SLOWPATH: u8 = 9;
pub const RET_PAUSE: u8 = 10;

#[inline(always)]
pub fn calculate_slot(addr: u64) -> usize {
    (addr as usize >> 2) & (TRACE_SIZE - 1)
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FixedTrace {
    pub address: u64,
    pub length: u32,
    pub cycles: u64,
    // We are using direct threaded code here:
    // https://en.wikipedia.org/wiki/Threaded_code
    // each individual thread is made of 2 consecutive
    // items here: the jumping offset, and the actual decoded
    // instructions. Since we will use both of them as plain
    // u64 values anyway in the assembly code, we cast the
    // Instruction type to u64. A test case will ensure that
    // Instruction type stays the same as u64 type.
    pub _threads: [u64; 2 * (TRACE_ITEM_LENGTH + 1)],
}

impl FixedTrace {
    pub fn thread(&self, idx: usize) -> Option<(Instruction, u64)> {
        if idx < TRACE_ITEM_LENGTH + 1 {
            Some((self._threads[idx * 2 + 1], self._threads[idx * 2]))
        } else {
            None
        }
    }

    pub fn set_thread(&mut self, idx: usize, instruction: Instruction, thread: u64) {
        if idx < TRACE_ITEM_LENGTH + 1 {
            self._threads[idx * 2] = thread;
            self._threads[idx * 2 + 1] = instruction;
        }
    }
}

impl Default for FixedTrace {
    fn default() -> Self {
        FixedTrace {
            address: 0,
            length: 0,
            cycles: 0,
            _threads: [0; 2 * (TRACE_ITEM_LENGTH + 1)],
        }
    }
}

#[repr(C)]
pub struct InvokeData {
    pub pause: *mut u8,
    pub fixed_traces: *const FixedTrace,
    pub fixed_trace_mask: u64,
}

// Although the memory here is an array, but when it is created,
//  its size is allocated through memory_size, and its maximum length RISCV_MAX_MEMORY
//  is used in the structure declaration.
#[repr(C)]
pub struct AsmCoreMachine {
    pub registers: [u64; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: u64,
    pub next_pc: u64,
    pub running: u8,
    pub cycles: u64,
    pub max_cycles: u64,
    pub chaos_mode: u8,
    pub chaos_seed: u32,
    pub load_reservation_address: u64,
    pub reset_signal: u8,
    pub isa: u8,
    pub version: u32,

    pub memory_size: u64,
    pub frames_size: u64,
    pub flags_size: u64,

    pub last_read_frame: u64,
    pub last_write_page: u64,

    pub memory_ptr: u64,
    pub flags_ptr: u64,
    pub frames_ptr: u64,
}

impl AsmCoreMachine {
    pub fn new(isa: u8, version: u32, max_cycles: u64) -> Box<AsmCoreMachine> {
        Self::new_with_memory(isa, version, max_cycles, RISCV_MAX_MEMORY)
    }

    pub fn new_with_memory(
        isa: u8,
        version: u32,
        max_cycles: u64,
        memory_size: usize,
    ) -> Box<AsmCoreMachine> {
        assert_ne!(memory_size, 0);
        assert!(memory_size <= RISCV_MAX_MEMORY);
        assert_eq!(memory_size % RISCV_PAGESIZE, 0);
        assert_eq!(memory_size % (1 << MEMORY_FRAME_SHIFTS), 0);
        let mut machine = unsafe {
            let layout = Layout::new::<AsmCoreMachine>();
            let raw_allocation = alloc(layout) as *mut AsmCoreMachine;
            Box::from_raw(raw_allocation)
        };
        machine.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
        machine.pc = 0;
        machine.next_pc = 0;
        machine.running = 0;
        machine.cycles = 0;
        machine.max_cycles = max_cycles;
        if cfg!(feature = "enable-chaos-mode-by-default") {
            machine.chaos_mode = 1;
        } else {
            machine.chaos_mode = 0;
        }
        machine.chaos_seed = 0;
        machine.reset_signal = 0;
        machine.version = version;
        machine.isa = isa;

        machine.memory_size = memory_size as u64;
        machine.frames_size = (memory_size / MEMORY_FRAMESIZE) as u64;
        machine.flags_size = (memory_size / RISCV_PAGESIZE) as u64;

        machine.last_read_frame = u64::max_value();
        machine.last_write_page = u64::max_value();

        machine
    }

    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.max_cycles = cycles;
    }
}

impl AsmCoreMachine {
    pub fn cast_ptr_to_slice(&self, ptr: u64, offset: usize, size: usize) -> &[u8] {
        unsafe {
            let ptr = ptr as *mut u8;
            let ptr = ptr.add(offset);
            std::slice::from_raw_parts(ptr, size)
        }
    }

    pub fn cast_ptr_to_slice_mut(&self, ptr: u64, offset: usize, size: usize) -> &mut [u8] {
        unsafe {
            let ptr = ptr as *mut u8;
            let ptr = ptr.add(offset);
            std::slice::from_raw_parts_mut(ptr, size)
        }
    }
}
