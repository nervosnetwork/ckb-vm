pub mod asm;
pub mod instructions;
pub mod memory;
pub mod registers;

pub const RISCV_PAGE_SHIFTS: usize = 12;
pub const RISCV_PAGESIZE: usize = 1 << RISCV_PAGE_SHIFTS;
pub const RISCV_GENERAL_REGISTER_NUMBER: usize = 32;
pub const MEMORY_FRAME_SHIFTS: usize = 18;
pub const MEMORY_FRAMESIZE: usize = 1 << MEMORY_FRAME_SHIFTS; // 256 KB
pub const MEMORY_FRAME_PAGE_SHIFTS: usize = MEMORY_FRAME_SHIFTS - RISCV_PAGE_SHIFTS;

pub const DEFAULT_MEMORY_SIZE: usize = 4 << 20; // 4 MB

pub const ISA_IMC: u8 = 0b0000_0000;
pub const ISA_B: u8 = 0b0000_0001;
pub const ISA_MOP: u8 = 0b0000_0010;
pub const ISA_A: u8 = 0b0000_0100;
