#![cfg(has_asm)]

use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{CoreMachine, VERSION0, VERSION1, VERSION2};
use ckb_vm::memory::{FLAG_EXECUTABLE, FLAG_FREEZED, Memory};
use ckb_vm::{
    DefaultMachineBuilder, DefaultMachineRunner, Error, ISA_B, ISA_IMC, ISA_MOP, RISCV_PAGESIZE,
    SupportMachine,
};

fn load_instructions(machine: &mut AsmMachine, instructions: &[u32]) {
    let bytes: Vec<u8> = instructions
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .collect();
    machine
        .machine
        .memory_mut()
        .init_pages(
            0x1000,
            RISCV_PAGESIZE as u64,
            FLAG_EXECUTABLE | FLAG_FREEZED,
            Some(bytes.into()),
            0,
        )
        .unwrap();
    machine.machine.update_pc(0x1000);
    machine.machine.commit_pc();
}

#[test]
fn reset_replaces_cached_instructions_and_preserves_rejection() {
    for version in [VERSION0, VERSION1, VERSION2] {
        let isa = if version == VERSION0 {
            ISA_IMC
        } else {
            ISA_IMC | ISA_B | ISA_MOP
        };
        let core = <Box<AsmCoreMachine> as SupportMachine>::new(isa, version, 3);
        let mut machine = AsmMachine::new(
            DefaultMachineBuilder::new(core)
                .instruction_cycle_func(Box::new(constant_cycles))
                .build(),
        );
        // addi a0, zero, value; addi a7, zero, 93; ecall
        load_instructions(&mut machine, &[0x0110_0513, 0x05d0_0893, 0x0000_0073]);
        assert_eq!(machine.run(), Ok(17));
        assert_eq!(machine.machine.cycles(), 3);

        machine.machine.reset(3);
        load_instructions(&mut machine, &[0x02a0_0513, 0x05d0_0893, 0x0000_0073]);
        assert_eq!(machine.run(), Ok(42));
        assert_eq!(machine.machine.cycles(), 3);

        machine.machine.reset(2);
        load_instructions(&mut machine, &[0x02a0_0513, 0x05d0_0893, 0x0000_0073]);
        assert_eq!(machine.run(), Err(Error::CyclesExceeded));

        machine.machine.reset(3);
        load_instructions(&mut machine, &[0xffff_ffff]);
        assert_eq!(
            machine.run(),
            Err(Error::InvalidInstruction {
                pc: 0x1000,
                instruction: 0xffff_ffff,
            })
        );
    }
}
