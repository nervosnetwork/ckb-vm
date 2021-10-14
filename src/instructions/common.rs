use crate::{
    instructions::{register::Register, utils::update_register, Error, RegisterIndex},
    machine::Machine,
    RISCV_MAX_MEMORY,
};

pub fn check_load_boundary<R: Register>(
    version0: bool,
    address: &R,
    bytes: u64,
) -> Result<(), Error> {
    if version0 {
        let address = address.to_u64();
        let end = address.checked_add(bytes).ok_or(Error::MemOutOfBound)?;
        if end == RISCV_MAX_MEMORY as u64 {
            return Err(Error::MemOutOfBound);
        }
    }
    Ok(())
}

pub fn set_vl<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    avl: u64,
    new_type: u64,
) -> Result<(), Error> {
    machine.set_vl(rd, rs1, avl, new_type);
    update_register(machine, rd, Mac::REG::from_u64(machine.vl()));
    Ok(())
}
