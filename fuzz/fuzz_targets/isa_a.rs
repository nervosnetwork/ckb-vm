#![no_main]
use ckb_vm::{CoreMachine, Memory};
use ckb_vm_fuzz_spike_ffi::Spike;
use libfuzzer_sys::fuzz_target;
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

    let insts: [u32; 18] = [
        0b00001_00_00000_00000_010_00000_0101111, // AMOSWAP.W
        0b00000_00_00000_00000_010_00000_0101111, // AMOADD.W
        0b00100_00_00000_00000_010_00000_0101111, // AMOXOR.W
        0b01100_00_00000_00000_010_00000_0101111, // AMOAND.W
        0b01000_00_00000_00000_010_00000_0101111, // AMOOR.W
        0b10000_00_00000_00000_010_00000_0101111, // AMOMIN.W
        0b10100_00_00000_00000_010_00000_0101111, // AMOMAX.W
        0b11000_00_00000_00000_010_00000_0101111, // AMOMINU.W
        0b11100_00_00000_00000_010_00000_0101111, // AMOMAXU.W
        0b00001_00_00000_00000_011_00000_0101111, // AMOSWAP.D
        0b00000_00_00000_00000_011_00000_0101111, // AMOADD.D
        0b00100_00_00000_00000_011_00000_0101111, // AMOXOR.D
        0b01100_00_00000_00000_011_00000_0101111, // AMOAND.D
        0b01000_00_00000_00000_011_00000_0101111, // AMOOR.D
        0b10000_00_00000_00000_011_00000_0101111, // AMOMIN.D
        0b10100_00_00000_00000_011_00000_0101111, // AMOMAX.D
        0b11000_00_00000_00000_011_00000_0101111, // AMOMINU.D
        0b11100_00_00000_00000_011_00000_0101111, // AMOMAXU.D
    ];

    for _ in 0..1024 {
        let inst = insts[deque.u8() as usize % insts.len()];
        let rs1 = deque.u8() as usize % 31 + 1;
        let rs2 = deque.u8() as usize % 31 + 1;
        let rs2_data = deque.u64();
        let rd = deque.u8() as usize % 32;

        spike.set_reg(rs1 as u64, 0x4000).unwrap();
        ckb_vm_int.set_register(rs1, 0x4000);
        ckb_vm_asm.set_register(rs1, 0x4000);

        if rs2 != rs1 {
            spike.set_reg(rs2 as u64, rs2_data).unwrap();
            ckb_vm_int.set_register(rs2, rs2_data);
            ckb_vm_asm.set_register(rs2, rs2_data);
        }

        let inst = inst | ((rs1 as u32) << 15) | ((rs2 as u32) << 20) | ((rd as u32) << 7);
        let insn = ckb_vm::instructions::a::factory::<u64>(inst, ckb_vm_version).unwrap();

        spike.execute(inst as u64).unwrap();
        ckb_vm::instructions::execute_instruction(insn, &mut ckb_vm_int).unwrap();
        ckb_vm::instructions::execute_instruction(insn, &mut ckb_vm_asm).unwrap();

        let spike_reg = spike.get_reg(rd as u64).unwrap();
        let ckb_vm_int_reg = ckb_vm_int.registers()[rd];
        let ckb_vm_asm_reg = ckb_vm_asm.registers()[rd];
        assert_eq!(spike_reg, ckb_vm_int_reg);
        assert_eq!(spike_reg, ckb_vm_asm_reg);

        let mut spike_mem = [0u8; 8];
        spike.ld(0x4000, 8, spike_mem.as_mut_ptr()).unwrap();
        let ckb_vm_int_mem = ckb_vm_int.memory_mut().load64(&0x4000).unwrap();
        assert_eq!(u64::from_le_bytes(spike_mem), ckb_vm_int_mem);
        let ckb_vm_asm_mem = ckb_vm_asm.memory_mut().load64(&0x4000).unwrap();
        assert_eq!(u64::from_le_bytes(spike_mem), ckb_vm_asm_mem);
    }
    for i in 0..32 {
        let spike_reg = spike.get_reg(i).unwrap();
        let ckb_vm_int_reg = ckb_vm_int.registers()[i as usize];
        let ckb_vm_asm_reg = ckb_vm_asm.registers()[i as usize];
        assert_eq!(spike_reg, ckb_vm_int_reg);
        assert_eq!(spike_reg, ckb_vm_asm_reg);
    }
});
