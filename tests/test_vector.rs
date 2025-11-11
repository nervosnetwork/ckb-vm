use ckb_vm::{
    ISA_IMC, ISA_V, Register, SupportMachine,
    instructions::{Itype, Rtype, handle_vadd_vv, handle_vsetvli, insts},
    machine::{CoreMachine, DefaultCoreMachine, RustDefaultMachineBuilder, VERSION2},
    memory::sparse::SparseMemory,
};

#[cfg(has_asm)]
use ckb_vm::DefaultMachineRunner;

#[cfg(has_asm)]
pub mod machine_build;

type Core = DefaultCoreMachine<u64, SparseMemory<u64>>;

fn build_machine() -> ckb_vm::machine::DefaultMachine<Core> {
    let core = Core::new(ISA_IMC | ISA_V, VERSION2, u64::MAX);
    RustDefaultMachineBuilder::new(core).build()
}

#[test]
fn vsetvli_updates_length_and_rd() {
    let mut machine = build_machine();
    machine.set_register(1, 5);
    let inst = Itype::new_u(insts::OP_VSETVLI, 2, 1, 3).0;
    handle_vsetvli(&mut machine, inst).expect("vsetvli executes");
    assert_eq!(machine.registers()[2].to_u64(), 2);
    assert_eq!(machine.vector_length(), Some(2));
}

#[test]
fn vadd_vv_adds_two_lanes() {
    let mut machine = build_machine();
    machine.set_register(1, 2);
    let vset = Itype::new_u(insts::OP_VSETVLI, 2, 1, 3).0;
    handle_vsetvli(&mut machine, vset).expect("vsetvli executes");
    {
        let vregs = machine.vector_registers_mut().expect("vector state");
        vregs[1] = [1, 2];
        vregs[2] = [3, 4];
    }
    let vadd = Rtype::new(insts::OP_VADD_VV, 3, 1, 2).0;
    handle_vadd_vv(&mut machine, vadd).expect("vadd.vv executes");
    let vregs = machine.vector_registers().expect("vector state");
    assert_eq!(vregs[3][0], 4);
    assert_eq!(vregs[3][1], 6);
}

#[test]
fn encodes_vsetvli_word() {
    let bits = 0x0035F057;
    assert!(
        ckb_vm::instructions::v::factory::<u64>(bits, VERSION2).is_some(),
        "vector factory failed"
    );
}

#[cfg(has_asm)]
#[test]
fn vector_binary_runs_on_asm() {
    let mut machine =
        machine_build::asm("tests/programs/vector_vadd", vec![], VERSION2, ISA_IMC | ISA_V);
    {
        let vregs = machine
            .machine_mut()
            .vector_registers_mut()
            .expect("vector state");
        vregs[1] = [1, 2];
        vregs[2] = [3, 4];
    }
    let exit_code = machine.run().expect("asm run");
    assert_eq!(exit_code, 0);
    let vregs = machine.machine().vector_registers().expect("vector state");
    assert_eq!(vregs[3][0], 4);
    assert_eq!(vregs[3][1], 6);
}
