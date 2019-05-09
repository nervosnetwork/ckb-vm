pub mod asm;
pub mod instructions;
pub mod registers;

pub const RISCV_PAGESIZE: usize = 1 << 12;
pub const RISCV_GENERAL_REGISTER_NUMBER: usize = 32;
// 4 MB
pub const RISCV_MAX_MEMORY: usize = 4 << 20;
// 1 MB
pub const DEFAULT_STACK_SIZE: usize = 1 << 20;
