// use bytes::Bytes;
// use ckb_vm::{
//     instructions::{blank_instruction, execute_instruction, is_basic_block_end_instruction},
//     CoreMachine, Error, Machine, Memory,
// };
// use proptest::prelude::*;

// pub struct DummyMachine {
//     pc: u64,
//     next_pc: u64,
//     registers: [u64; 32],
//     can_touch_pc: bool,
// }

// impl CoreMachine for DummyMachine {
//     type REG = u64;
//     type MEM = Self;

//     fn pc(&self) -> &Self::REG {
//         if !self.can_touch_pc {
//             panic!("Instruction is affecting pc!");
//         }
//         &self.pc
//     }

//     fn update_pc(&mut self, pc: Self::REG) {
//         if !self.can_touch_pc {
//             panic!("Instruction is affecting pc!");
//         }
//         self.next_pc = pc;
//     }

//     fn commit_pc(&mut self) {
//         if !self.can_touch_pc {
//             panic!("Instruction is affecting pc!");
//         }
//         self.pc = self.next_pc;
//     }

//     fn memory(&self) -> &Self::MEM {
//         self
//     }

//     fn memory_mut(&mut self) -> &mut Self::MEM {
//         self
//     }

//     fn registers(&self) -> &[Self::REG] {
//         &self.registers
//     }

//     fn set_register(&mut self, idx: usize, value: Self::REG) {
//         self.registers[idx] = value;
//     }

//     fn element_ref(&self, _reg: usize, _sew: u64, _n: usize) -> &[u8] {
//         unreachable!()
//     }

//     fn element_mut(&mut self, _reg: usize, _sew: u64, _n: usize) -> &mut [u8] {
//         unreachable!()
//     }

//     fn get_bit(&self, _reg: usize, _n: usize) -> bool {
//         unreachable!()
//     }

//     fn set_bit(&mut self, _reg: usize, _n: usize) {
//         unreachable!()
//     }

//     fn clr_bit(&mut self, _reg: usize, _n: usize) {
//         unreachable!()
//     }

//     fn set_vl(&mut self, _rd: usize, _rs1: usize, _avl: u64, _new_type: u64) {}

//     fn vl(&self) -> u64 {
//         0
//     }

//     fn vlmax(&self) -> u64 {
//         unreachable!()
//     }

//     fn vsew(&self) -> u64 {
//         unreachable!()
//     }

//     fn vlmul(&self) -> f64 {
//         unreachable!()
//     }

//     fn vta(&self) -> bool {
//         unreachable!()
//     }

//     fn vma(&self) -> bool {
//         unreachable!()
//     }

//     fn vill(&self) -> bool {
//         true
//     }

//     fn vlenb(&self) -> u64 {
//         unreachable!()
//     }

// fn v_to_mem(
//     &mut self,
//     _reg: usize,
//     _sew: u64,
//     _skip: usize,
//     _count: usize,
//     _addr: u64,
// ) -> Result<(), Error> {
//     unreachable!()
// }

// fn mem_to_v(
//     &mut self,
//     _reg: usize,
//     _sew: u64,
//     _skip: usize,
//     _count: usize,
//     _addr: u64,
// ) -> Result<(), Error> {
//     unreachable!()
// }

// fn v_to_v(
//     &mut self,
//     _reg: usize,
//     _sew: u64,
//     _skip: usize,
//     _count: usize,
//     _target_reg: usize,
// ) -> Result<(), Error> {
//     unreachable!()
// }

//     fn version(&self) -> u32 {
//         1
//     }

//     fn isa(&self) -> u8 {
//         unreachable!()
//     }
// }

// impl Machine for DummyMachine {
//     fn ecall(&mut self) -> Result<(), Error> {
//         Ok(())
//     }

//     fn ebreak(&mut self) -> Result<(), Error> {
//         Ok(())
//     }
// }

// impl Memory for DummyMachine {
//     type REG = u64;

//     fn init_pages(
//         &mut self,
//         _addr: u64,
//         _size: u64,
//         _flags: u8,
//         _source: Option<Bytes>,
//         _offset_from_addr: u64,
//     ) -> Result<(), Error> {
//         unreachable!()
//     }

//     fn fetch_flag(&mut self, _page: u64) -> Result<u8, Error> {
//         unreachable!()
//     }

//     fn set_flag(&mut self, _page: u64, _flag: u8) -> Result<(), Error> {
//         unreachable!()
//     }

//     fn clear_flag(&mut self, _page: u64, _flag: u8) -> Result<(), Error> {
//         unreachable!()
//     }

//     fn store_byte(&mut self, _addr: u64, _size: u64, _value: u8) -> Result<(), Error> {
//         unreachable!()
//     }

//     fn store_bytes(&mut self, _addr: u64, _value: &[u8]) -> Result<(), Error> {
//         unreachable!()
//     }

//     fn execute_load16(&mut self, _addr: u64) -> Result<u16, Error> {
//         unreachable!()
//     }

//     fn execute_load32(&mut self, _addr: u64) -> Result<u32, Error> {
//         unreachable!()
//     }

//     fn load_bytes(&mut self, _addr: u64, _size: u64) -> Result<Vec<u8>, Error> {
//         unreachable!()
//     }

//     fn load8(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
//         Ok(8)
//     }

//     fn load16(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
//         Ok(16)
//     }

//     fn load32(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
//         Ok(32)
//     }

//     fn load64(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
//         Ok(64)
//     }

//     fn store8(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
//         Ok(())
//     }

//     fn store16(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
//         Ok(())
//     }

//     fn store32(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
//         Ok(())
//     }

//     fn store64(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
//         Ok(())
//     }
// }

// proptest! {
//     #[test]
//     fn only_basic_block_instruction_can_touch_pc(pc: u64, registers: [u64; 31], opcode: u16) {
//         let mut full_registers = [0u64; 32];
//         full_registers[1..32].copy_from_slice(&registers);
//         let inst = blank_instruction(opcode);

//         let mut machine = DummyMachine {
//             pc,
//             next_pc: 0,
//             registers: full_registers,
//             can_touch_pc: is_basic_block_end_instruction(inst),
//         };
//         match execute_instruction(inst, &mut machine) {
//             Ok(_) => (),
//             Err(Error::InvalidOp(_)) => (),
//             Err(e) => panic!("Execute error: {}", e),
//         }
//     }
// }

// #[test]
// fn test_basic_block_end_imc_inst_can_touch_pc() {
//     for opcode in 0u16..0x100u16 {
//         let inst = blank_instruction(opcode);

//         let mut machine = DummyMachine {
//             pc: 0x10000,
//             next_pc: 0,
//             registers: [0u64; 32],
//             can_touch_pc: is_basic_block_end_instruction(inst),
//         };
//         match execute_instruction(inst, &mut machine) {
//             Ok(_) => (),
//             Err(Error::InvalidOp(_)) => (),
//             Err(e) => panic!("Execute error: {}", e),
//         }
//     }
// }
