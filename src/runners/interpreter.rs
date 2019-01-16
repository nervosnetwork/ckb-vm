use crate::{
    decoder::build_imac_decoder, CoreMachine, DefaultMachine, Error, Memory, Register,
    SupportMachine,
};

pub fn run<'a, R: Register, M: Memory>(
    machine: &mut DefaultMachine<'a, R, M>,
) -> Result<u8, Error> {
    let decoder = build_imac_decoder::<R>();
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
