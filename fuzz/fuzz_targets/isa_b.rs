#![no_main]
use ckb_vm::CoreMachine;
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
    let ckb_vm_isa = ckb_vm::ISA_IMC | ckb_vm::ISA_A | ckb_vm::ISA_B;
    let ckb_vm_version = ckb_vm::machine::VERSION2;
    let mut ckb_vm_int =
        ckb_vm::DefaultMachineBuilder::new(ckb_vm::DefaultCoreMachine::<
            u64,
            ckb_vm::SparseMemory<u64>,
        >::new(ckb_vm_isa, ckb_vm_version, u64::MAX))
        .build();
    let mut ckb_vm_asm = ckb_vm::DefaultMachineBuilder::new(
        ckb_vm::machine::asm::AsmCoreMachine::new(ckb_vm_isa, ckb_vm_version, u64::MAX),
    )
    .build();

    #[rustfmt::skip]
    let insts: [(u32, u32); 43] = [
        (0b0000100_00000_00000_000_00000_0111011, 0b0000000_11111_11111_000_11111_0000000), // ADDUW
        (0b0100000_00000_00000_111_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // ANDN
        (0b0100100_00000_00000_001_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // BCLR
        (0b010010_000000_00000_001_00000_0010011, 0b000000_111111_11111_000_11111_0000000), // BCLRI
        (0b0100100_00000_00000_101_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // BEXT
        (0b010010_000000_00000_101_00000_0010011, 0b000000_111111_11111_000_11111_0000000), // BEXTI
        (0b0110100_00000_00000_001_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // BINV
        (0b011010_000000_00000_001_00000_0010011, 0b000000_111111_11111_000_11111_0000000), // BINVI
        (0b0010100_00000_00000_001_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // BSET
        (0b001010_000000_00000_001_00000_0010011, 0b000000_111111_11111_000_11111_0000000), // BSETI
        (0b0000101_00000_00000_001_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // CLMUL
        (0b0000101_00000_00000_011_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // CLMULH
        (0b0000101_00000_00000_010_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // CLMULR
        (0b0110000_00000_00000_001_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // CLZ
        (0b0110000_00000_00000_001_00000_0011011, 0b0000000_00000_11111_000_11111_0000000), // CLZW
        (0b0110000_00010_00000_001_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // CPOP
        (0b0110000_00010_00000_001_00000_0011011, 0b0000000_00000_11111_000_11111_0000000), // CPOPW
        (0b0110000_00001_00000_001_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // CTZ
        (0b0110000_00001_00000_001_00000_0011011, 0b0000000_00000_11111_000_11111_0000000), // CTZW
        (0b0000101_00000_00000_110_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // MAX
        (0b0000101_00000_00000_111_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // MAXU
        (0b0000101_00000_00000_100_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // MIN
        (0b0000101_00000_00000_101_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // MINU
        (0b0010100_00111_00000_101_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // ORCB
        (0b0100000_00000_00000_110_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // ORN
        (0b0110101_11000_00000_101_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // REV8
        (0b0110000_00000_00000_001_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // ROL
        (0b0110000_00000_00000_001_00000_0111011, 0b0000000_11111_11111_000_11111_0000000), // ROLW
        (0b0110000_00000_00000_101_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // ROR
        (0b011000_000000_00000_101_00000_0010011, 0b000000_111111_11111_000_11111_0000000), // RORI
        (0b0110000_00000_00000_101_00000_0011011, 0b0000000_11111_11111_000_11111_0000000), // RORIW
        (0b0110000_00000_00000_101_00000_0111011, 0b0000000_11111_11111_000_11111_0000000), // RORW
        (0b0110000_00100_00000_001_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // SEXTB
        (0b0110000_00101_00000_001_00000_0010011, 0b0000000_00000_11111_000_11111_0000000), // SEXTH
        (0b0010000_00000_00000_010_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // SH1ADD
        (0b0010000_00000_00000_010_00000_0111011, 0b0000000_11111_11111_000_11111_0000000), // SH1ADDUW
        (0b0010000_00000_00000_100_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // SH2ADD
        (0b0010000_00000_00000_100_00000_0111011, 0b0000000_11111_11111_000_11111_0000000), // SH2ADDUW
        (0b0010000_00000_00000_110_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // SH3ADD
        (0b0010000_00000_00000_110_00000_0111011, 0b0000000_11111_11111_000_11111_0000000), // SH3ADDUW
        (0b000010_000000_00000_001_00000_0011011, 0b000000_111111_11111_000_11111_0000000), // SLLIUW
        (0b0100000_00000_00000_100_00000_0110011, 0b0000000_11111_11111_000_11111_0000000), // XNOR
        (0b0000100_00000_00000_100_00000_0111011, 0b0000000_00000_11111_000_11111_0000000), // ZEXTH
    ];
    for i in 1..32 {
        let d = deque.u64();
        spike.set_reg(i as u64, d).unwrap();
        ckb_vm_int.set_register(i, d);
        ckb_vm_asm.set_register(i, d);
    }
    for _ in 0..1024 {
        let choose = deque.u8() as usize % insts.len();
        let inst = insts[choose].0;
        let mask = insts[choose].1;

        let inst = inst | (mask & deque.u32());
        let insn = ckb_vm::instructions::b::factory::<u64>(inst, ckb_vm_version).unwrap();

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