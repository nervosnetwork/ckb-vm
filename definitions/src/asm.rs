use crate::{
    instructions::Instruction, MEMORY_FRAMES, MEMORY_FRAMESIZE, MEMORY_FRAME_SHIFTS,
    RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGES, RISCV_PAGESIZE,
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

#[inline(always)]
pub fn calculate_slot(addr: u64) -> usize {
    (addr as usize >> 2) & (TRACE_SIZE - 1)
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

    pub flags: [u8; RISCV_PAGES],
    pub frames: [u8; MEMORY_FRAMES],
    pub traces: [Trace; TRACE_SIZE],

    pub memory: [u8; RISCV_MAX_MEMORY],
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
            let machine_size =
                std::mem::size_of::<AsmCoreMachine>() - RISCV_MAX_MEMORY + memory_size;

            let layout = Layout::array::<u8>(machine_size).unwrap();
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
        machine.flags = [0; RISCV_PAGES];
        for i in 0..TRACE_SIZE {
            machine.traces[i] = Trace::default();
        }
        machine.frames = [0; MEMORY_FRAMES];

        machine.memory_size = memory_size as u64;
        machine.frames_size = (memory_size / MEMORY_FRAMESIZE) as u64;
        machine.flags_size = (memory_size / RISCV_PAGESIZE) as u64;

        machine.last_read_frame = u64::max_value();
        machine.last_write_page = u64::max_value();

        machine
    }
}
