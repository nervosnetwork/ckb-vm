use crate::{
    instructions::Instruction, MEMORY_FRAMES, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY,
    RISCV_PAGES,
};

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

impl AsRef<Box<AsmCoreMachine>> for Box<AsmCoreMachine> {
    #[inline(always)]
    fn as_ref(&self) -> &Box<AsmCoreMachine> {
        self
    }
}

impl AsMut<Box<AsmCoreMachine>> for Box<AsmCoreMachine> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Box<AsmCoreMachine> {
        self
    }
}
