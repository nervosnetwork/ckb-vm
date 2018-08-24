use super::super::machine::Machine;
use super::super::memory::Memory;
use super::utils::{update_register};
use super::RegisterIndex;


// Other instruction set functions common with RVC

pub fn add<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers[rs1];
    let rs2_value = machine.registers[rs2];
    let (value, _) = rs1_value.overflowing_add(rs2_value);
    update_register(machine, rd, value);
}
