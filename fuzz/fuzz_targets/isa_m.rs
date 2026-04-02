#![no_main]
use ckb_vm::{CoreMachine, SupportMachine};
use libfuzzer_sys::fuzz_target;
use spike_sys::Spike;
use std::collections::VecDeque;

struct Deque {
    n: VecDeque<u8>,
}

impl Deque {
    fn new(data: [u8; 512]) -> Self {
        Self {
            n: VecDeque::from(data),
        }
    }

    fn u8(&mut self) -> u8 {
        let r = self.n.pop_front().unwrap();
        self.n.push_back(r);
        r
    }

    fn u32(&mut self) -> u32 {
        let mut r = [0u8; 4];
        r.fill_with(|| self.u8());
        u32::from_le_bytes(r)
    }

    fn u64(&mut self) -> u64 {
        let mut r = [0u8; 8];
        r.fill_with(|| self.u8());
        u64::from_le_bytes(r)
    }
}

fuzz_target!(|data: [u8; 512]| {
    let mut deque = Deque::new(data);
    let spike = Spike::new(4 * 1024 * 1024 - 4096);
    let ckb_vm_isa = ckb_vm::ISA_IMC;
    let ckb_vm_version = ckb_vm::machine::VERSION2;
    let mut ckb_vm_int =
        ckb_vm::RustDefaultMachineBuilder::new(ckb_vm::DefaultCoreMachine::<
            u64,
            ckb_vm::SparseMemory<u64>,
        >::new(ckb_vm_isa, ckb_vm_version, u64::MAX))
        .build();
    let mut ckb_vm_asm = ckb_vm::machine::asm::AsmDefaultMachineBuilder::new(
        <ckb_vm::machine::asm::AsmCoreMachine as SupportMachine>::new(
            ckb_vm_isa,
            ckb_vm_version,
            u64::MAX,
        ),
    )
    .build();

    // Mask covering rs2[24:20], rs1[19:15], and rd[11:7] — leaves funct7 and funct3 intact.
    let mask = 0b0000000_11111_11111_000_11111_0000000u32;
    #[rustfmt::skip]
    let insts: [(u32, u32); 13] = [
        (0b0000001_00000_00000_000_00000_0110011, mask), // MUL
        (0b0000001_00000_00000_001_00000_0110011, mask), // MULH
        (0b0000001_00000_00000_010_00000_0110011, mask), // MULHSU
        (0b0000001_00000_00000_011_00000_0110011, mask), // MULHU
        (0b0000001_00000_00000_100_00000_0110011, mask), // DIV
        (0b0000001_00000_00000_101_00000_0110011, mask), // DIVU
        (0b0000001_00000_00000_110_00000_0110011, mask), // REM
        (0b0000001_00000_00000_111_00000_0110011, mask), // REMU
        (0b0000001_00000_00000_000_00000_0111011, mask), // MULW
        (0b0000001_00000_00000_100_00000_0111011, mask), // DIVW
        (0b0000001_00000_00000_101_00000_0111011, mask), // DIVUW
        (0b0000001_00000_00000_110_00000_0111011, mask), // REMW
        (0b0000001_00000_00000_111_00000_0111011, mask), // REMUW
    ];

    for i in 1..32 {
        let d = deque.u64();
        spike.set_reg(i as u64, d).unwrap();
        ckb_vm_int.set_register(i, d);
        ckb_vm_asm.set_register(i, d);
    }
    for _ in 0..1024 {
        let choose = deque.u8() as usize % insts.len();
        let inst = insts[choose].0 | (insts[choose].1 & deque.u32());
        let insn = ckb_vm::instructions::m::factory::<u64>(inst, ckb_vm_version).unwrap();

        spike.execute(inst as u64).unwrap();
        ckb_vm::instructions::execute_instruction(insn, &mut ckb_vm_int).unwrap();
        ckb_vm::instructions::execute_instruction(insn, &mut ckb_vm_asm).unwrap();
    }
    for i in 0..32 {
        let spike_reg = spike.get_reg(i).unwrap();
        let ckb_vm_int_reg = ckb_vm_int.registers()[i as usize];
        let ckb_vm_asm_reg = ckb_vm_asm.registers()[i as usize];
        assert_eq!(spike_reg, ckb_vm_int_reg);
        assert_eq!(spike_reg, ckb_vm_asm_reg);
    }
});
