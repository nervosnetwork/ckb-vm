use crate::{
    decoder::build_imac_decoder, CoreMachine, DefaultMachine, Error, Register, SupportMachine,
};

pub fn run<Inner: SupportMachine>(machine: &mut DefaultMachine<Inner>) -> Result<u8, Error> {
    let decoder = build_imac_decoder::<Inner::REG>();
    machine.set_running(true);
    while machine.running() {
        let instruction = {
            let pc = machine.pc().to_usize();
            let memory = machine.memory_mut();
            decoder.decode(memory, pc)?
        };
        instruction.execute(machine)?;
        let cycles = machine
            .instruction_cycle_func()
            .as_ref()
            .map(|f| f(&instruction))
            .unwrap_or(0);
        machine.add_cycles(cycles)?;
    }
    Ok(machine.exit_code())
}
