use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::RISCV_MAX_MEMORY;
use super::register::Register;
use super::utils::update_register;
use super::v_register::{VRegister, U1024, U128, U16, U256, U32, U512, U64, U8};
use super::{Error, Immediate, RegisterIndex, UImmediate};

// Other instruction set functions common with RVC

// ======================
// #  ALU instructions  #
// ======================
pub fn add<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_add(&rs2_value);
    update_register(machine, rd, value);
}

pub fn addw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_add(&rs2_value);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn sub<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_sub(&rs2_value);
    update_register(machine, rd, value);
}

pub fn subw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_sub(&rs2_value);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn addi<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    update_register(machine, rd, value);
}

pub fn addiw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

// =======================
// #  LOAD instructions  #
// =======================
fn check_load_boundary<R: Register>(version0: bool, address: &R, bytes: u64) -> Result<(), Error> {
    if version0 {
        let address = address.to_u64();
        let end = address.checked_add(bytes).ok_or(Error::OutOfBound)?;
        if end == RISCV_MAX_MEMORY as u64 {
            return Err(Error::OutOfBound);
        }
    }
    Ok(())
}

pub fn lb<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 1)?;
    let value = machine.memory_mut().load8(&address)?;
    // sign-extened
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(8)));
    Ok(())
}

pub fn lh<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 2)?;
    let value = machine.memory_mut().load16(&address)?;
    // sign-extened
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(16)));
    Ok(())
}

pub fn lw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 4)?;
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn ld<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 8)?;
    let value = machine.memory_mut().load64(&address)?;
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(64)));
    Ok(())
}

pub fn lbu<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 1)?;
    let value = machine.memory_mut().load8(&address)?;
    update_register(machine, rd, value);
    Ok(())
}

pub fn lhu<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 2)?;
    let value = machine.memory_mut().load16(&address)?;
    update_register(machine, rd, value);
    Ok(())
}

pub fn lwu<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 4)?;
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, rd, value);
    Ok(())
}

// ========================
// #  STORE instructions  #
// ========================
pub fn sb<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store8(&address, &value)?;
    Ok(())
}

pub fn sh<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store16(&address, &value)?;
    Ok(())
}

pub fn sw<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store32(&address, &value)?;
    Ok(())
}

pub fn sd<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store64(&address, &value)?;
    Ok(())
}

// =========================
// #  BIT-OP instructions  #
// =========================
pub fn and<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1 as usize].clone();
    let rs2_value = machine.registers()[rs2 as usize].clone();
    let value = rs1_value & rs2_value;
    update_register(machine, rd, value);
}

pub fn xor<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1 as usize].clone();
    let rs2_value = machine.registers()[rs2 as usize].clone();
    let value = rs1_value ^ rs2_value;
    update_register(machine, rd, value);
}

pub fn or<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1 as usize].clone();
    let rs2_value = machine.registers()[rs2 as usize].clone();
    let value = rs1_value | rs2_value;
    update_register(machine, rd, value);
}

pub fn andi<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1 as usize].clone() & Mac::REG::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn xori<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1 as usize].clone() ^ Mac::REG::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn ori<Mac: Machine>(machine: &mut Mac, rd: RegisterIndex, rs1: RegisterIndex, imm: Immediate) {
    let value = machine.registers()[rs1 as usize].clone() | Mac::REG::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn slli<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() << Mac::REG::from_u32(shamt);
    update_register(machine, rd, value);
}

pub fn srli<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() >> Mac::REG::from_u32(shamt);
    update_register(machine, rd, value);
}

pub fn srai<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].signed_shr(&Mac::REG::from_u32(shamt));
    update_register(machine, rd, value);
}

pub fn slliw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() << Mac::REG::from_u32(shamt);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn srliw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].zero_extend(&Mac::REG::from_u8(32))
        >> Mac::REG::from_u32(shamt);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn sraiw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize]
        .sign_extend(&Mac::REG::from_u8(32))
        .signed_shr(&Mac::REG::from_u32(shamt));
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

// =======================
// #  JUMP instructions  #
// =======================
pub fn jal<Mac: Machine>(machine: &mut Mac, rd: RegisterIndex, imm: Immediate, xbytes: u8) {
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(xbytes));
    update_register(machine, rd, link);
    machine.update_pc(machine.pc().overflowing_add(&Mac::REG::from_i32(imm)));
}

// ==================
// #  vset{i}vl{i}  #
// ==================
pub fn set_vl<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    reqvl: u32,
    new_type: u32,
) -> Result<(), Error> {
    let old_vsew = machine.get_vsew();
    machine.set_vl(rd, rs1, Mac::REG::from_u32(reqvl), new_type);
    update_register(machine, rd, Mac::REG::from_u32(machine.get_vl()));
    // https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#344-vector-type-illegal-vill
    //
    // > If the vill bit is set, then any attempt to execute a vector instruction that
    // > depends upon vtype will raise an illegal-instruction exception.
    //
    // Check vill right here can reduce repeated vill checks.
    if machine.get_vill() {
        return Err(Error::Vill);
    }

    let new_vsew = machine.get_vsew();
    if old_vsew != new_vsew {
        for i in 0..32 {
            let fr = &machine.vregisters()[i];
            let le_byte: [u8; 256] = match fr {
                VRegister::U8(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        r[i] = e.0;
                    }
                    r
                }
                VRegister::U16(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 2;
                        let end = (i + 1) * 2;
                        r[start..end].copy_from_slice(&e.0.to_le_bytes());
                    }
                    r
                }
                VRegister::U32(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 4;
                        let end = (i + 1) * 4;
                        r[start..end].copy_from_slice(&e.0.to_le_bytes());
                    }
                    r
                }
                VRegister::U64(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 8;
                        let end = (i + 1) * 8;
                        r[start..end].copy_from_slice(&e.0.to_le_bytes());
                    }
                    r
                }
                VRegister::U128(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 16;
                        let end = (i + 1) * 16;
                        r[start..end].copy_from_slice(&e.0.to_le_bytes());
                    }
                    r
                }
                VRegister::U256(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 32;
                        let end = (i + 1) * 32;
                        r[start..end].copy_from_slice(&e.to_le_bytes());
                    }
                    r
                }
                VRegister::U512(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 64;
                        let end = (i + 1) * 64;
                        r[start..end].copy_from_slice(&e.to_le_bytes());
                    }
                    r
                }
                VRegister::U1024(data) => {
                    let mut r = [0x00; 256];
                    for (i, e) in data.iter().enumerate() {
                        let start = i * 128;
                        let end = (i + 1) * 128;
                        r[start..end].copy_from_slice(&e.to_le_bytes());
                    }
                    r
                }
            };
            let tr = match new_vsew {
                8 => {
                    let mut r = [U8::default(); 256];
                    for i in 0..256 {
                        r[i] = U8(le_byte[i])
                    }
                    VRegister::U8(r)
                }
                16 => {
                    let mut buf = [0x00; 2];
                    let mut r = [U16::default(); 128];
                    for i in 0..128 {
                        buf.copy_from_slice(&le_byte[i * 2..(i + 1) * 2]);
                        r[i] = U16::from_le_bytes(buf);
                    }
                    VRegister::U16(r)
                }
                32 => {
                    let mut buf = [0x00; 4];
                    let mut r = [U32::default(); 64];
                    for i in 0..64 {
                        buf.copy_from_slice(&le_byte[i * 4..(i + 1) * 4]);
                        r[i] = U32::from_le_bytes(buf);
                    }
                    VRegister::U32(r)
                }
                64 => {
                    let mut buf = [0x00; 8];
                    let mut r = [U64::default(); 32];
                    for i in 0..32 {
                        buf.copy_from_slice(&le_byte[i * 8..(i + 1) * 8]);
                        r[i] = U64::from_le_bytes(buf);
                    }
                    VRegister::U64(r)
                }
                128 => {
                    let mut buf = [0x00; 16];
                    let mut r = [U128::default(); 16];
                    for i in 0..16 {
                        buf.copy_from_slice(&le_byte[i * 16..(i + 1) * 16]);
                        r[i] = U128::from_le_bytes(buf);
                    }
                    VRegister::U128(r)
                }
                256 => {
                    let mut buf = [0x00; 32];
                    let mut r = [U256::default(); 8];
                    for i in 0..8 {
                        buf.copy_from_slice(&le_byte[i * 32..(i + 1) * 32]);
                        r[i] = U256::from_le_bytes(buf);
                    }
                    VRegister::U256(r)
                }
                512 => {
                    let mut buf = [0x00; 64];
                    let mut r = [U512::default(); 4];
                    for i in 0..4 {
                        buf.copy_from_slice(&le_byte[i * 64..(i + 1) * 64]);
                        r[i] = U512::from_le_bytes(buf);
                    }
                    VRegister::U512(r)
                }
                1024 => {
                    let mut buf = [0x00; 128];
                    let mut r = [U1024::default(); 2];
                    for i in 0..2 {
                        buf.copy_from_slice(&le_byte[i * 128..(i + 1) * 128]);
                        r[i] = U1024::from_le_bytes(buf);
                    }
                    VRegister::U1024(r)
                }
                _ => unreachable!(),
            };
            machine.set_vregister(i, tr);
        }
    }

    Ok(())
}
