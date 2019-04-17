pub mod asm;
pub mod instructions;
pub mod registers;

pub const RISCV_PAGESIZE: usize = 1 << 12;
pub const RISCV_GENERAL_REGISTER_NUMBER: usize = 32;
// 16 MB
pub const RISCV_MAX_MEMORY: usize = 16 << 20;
// 2 MB
pub const DEFAULT_STACK_SIZE: usize = 2 << 20;
