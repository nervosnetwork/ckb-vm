use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::RISCV_MAX_MEMORY;
use super::register::Register;
use super::utils::update_register;
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
pub fn jal<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    imm: Immediate,
    xbytes: u8,
) -> Option<Mac::REG> {
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(xbytes));
    update_register(machine, rd, link);
    Some(machine.pc().overflowing_add(&Mac::REG::from_i32(imm)))
}

// ==============================
// #  B-extension instructions  #
// ==============================
pub fn gorc32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x01) != 0 {
        x |= ((x & 0x5555_5555) << 0x01) | ((x & 0xAAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x |= ((x & 0x3333_3333) << 0x02) | ((x & 0xCCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x |= ((x & 0x0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x |= ((x & 0x00FF_00FF) << 0x08) | ((x & 0xFF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x |= ((x & 0x0000_FFFF) << 0x10) | ((x & 0xFFFF_0000) >> 0x10);
    }
    x
}

pub fn gorc64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x3f;
    if (shamt & 0x01) != 0 {
        x |= ((x & 0x5555_5555_5555_5555) << 0x01) | ((x & 0xAAAA_AAAA_AAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x |= ((x & 0x3333_3333_3333_3333) << 0x02) | ((x & 0xCCCC_CCCC_CCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x |= ((x & 0x0F0F_0F0F_0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0_F0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x |= ((x & 0x00FF_00FF_00FF_00FF) << 0x08) | ((x & 0xFF00_FF00_FF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x |= ((x & 0x0000_FFFF_0000_FFFF) << 0x10) | ((x & 0xFFFF_0000_FFFF_0000) >> 0x10);
    }
    if (shamt & 0x20) != 0 {
        x |= ((x & 0x0000_0000_FFFF_FFFF) << 0x20) | ((x & 0xFFFF_FFFF_0000_0000) >> 0x20);
    }
    x
}

pub fn grev32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x01) != 0 {
        x = ((x & 0x5555_5555) << 0x01) | ((x & 0xAAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x = ((x & 0x3333_3333) << 0x02) | ((x & 0xCCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x = ((x & 0x0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x = ((x & 0x00FF_00FF) << 0x08) | ((x & 0xFF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x = ((x & 0x0000_FFFF) << 0x10) | ((x & 0xFFFF_0000) >> 0x10);
    }
    x
}

pub fn grev64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x3f;
    if (shamt & 0x01) != 0 {
        x = ((x & 0x5555_5555_5555_5555) << 0x01) | ((x & 0xAAAA_AAAA_AAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x = ((x & 0x3333_3333_3333_3333) << 0x02) | ((x & 0xCCCC_CCCC_CCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x = ((x & 0x0F0F_0F0F_0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0_F0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x = ((x & 0x00FF_00FF_00FF_00FF) << 0x08) | ((x & 0xFF00_FF00_FF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x = ((x & 0x0000_FFFF_0000_FFFF) << 0x10) | ((x & 0xFFFF_0000_FFFF_0000) >> 0x10);
    }
    if (shamt & 0x20) != 0 {
        x = ((x & 0x0000_0000_FFFF_FFFF) << 0x20) | ((x & 0xFFFF_FFFF_0000_0000) >> 0x20);
    }
    x
}

pub fn bext32(rs1: u32, rs2: u32) -> u32 {
    let mut r: u32 = 0;
    let mut j = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> i) & 1) != 0 {
                r |= 1 << j;
            }
            j += 1;
        }
    }
    r
}

pub fn bext64(rs1: u64, rs2: u64) -> u64 {
    let mut r: u64 = 0;
    let mut j = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> i) & 1) != 0 {
                r |= 1 << j;
            }
            j += 1;
        }
    }
    r
}

pub fn bdep32(rs1: u32, rs2: u32) -> u32 {
    let mut r: u32 = 0;
    let mut j = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> j) & 1) != 0 {
                r |= 1 << i;
            }
            j += 1;
        }
    }
    r
}

pub fn bdep64(rs1: u64, rs2: u64) -> u64 {
    let mut r: u64 = 0;
    let mut j = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> j) & 1) != 0 {
                r |= 1 << i;
            }
            j += 1;
        }
    }
    r
}

pub fn clmul32(rs1: u32, rs2: u32) -> u32 {
    let mut x: u32 = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 << i;
        }
    }
    x
}

pub fn clmul64(rs1: u64, rs2: u64) -> u64 {
    let mut x: u64 = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 << i;
        }
    }
    x
}

pub fn clmulh32(rs1: u32, rs2: u32) -> u32 {
    let mut x: u32 = 0;
    for i in 1..32 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (32 - i);
        }
    }
    x
}

pub fn clmulh64(rs1: u64, rs2: u64) -> u64 {
    let mut x: u64 = 0;
    for i in 1..64 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (64 - i);
        }
    }
    x
}

pub fn clmulr32(rs1: u32, rs2: u32) -> u32 {
    let mut x: u32 = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (31 - i);
        }
    }
    x
}

pub fn clmulr64(rs1: u64, rs2: u64) -> u64 {
    let mut x: u64 = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (63 - i);
        }
    }
    x
}

fn shuffle32_stage(src: u32, maskl: u32, maskr: u32, n: u32) -> u32 {
    let mut x = src & !(maskl | maskr);
    x |= ((src << n) & maskl) | ((src >> n) & maskr);
    x
}

fn shuffle64_stage(src: u64, maskl: u64, maskr: u64, n: u64) -> u64 {
    let mut x = src & !(maskl | maskr);
    x |= ((src << n) & maskl) | ((src >> n) & maskr);
    x
}

pub fn shfl32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 15;
    if (shamt & 8) != 0 {
        x = shuffle32_stage(x, 0x00FF_0000, 0x0000_FF00, 8);
    }
    if (shamt & 4) != 0 {
        x = shuffle32_stage(x, 0x0F00_0F00, 0x00F0_00F0, 4);
    }
    if (shamt & 2) != 0 {
        x = shuffle32_stage(x, 0x3030_3030, 0x0C0C_0C0C, 2);
    }
    if (shamt & 1) != 0 {
        x = shuffle32_stage(x, 0x4444_4444, 0x2222_2222, 1);
    }
    x
}

pub fn shfl64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x10) != 0 {
        x = shuffle64_stage(x, 0x0000_FFFF_0000_0000, 0x0000_0000_FFFF_0000, 0x10);
    }
    if (shamt & 0x08) != 0 {
        x = shuffle64_stage(x, 0x00FF_0000_00FF_0000, 0x0000_FF00_0000_FF00, 0x08);
    }
    if (shamt & 0x04) != 0 {
        x = shuffle64_stage(x, 0x0F00_0F00_0F00_0F00, 0x00F0_00F0_00F0_00F0, 0x04);
    }
    if (shamt & 0x02) != 0 {
        x = shuffle64_stage(x, 0x3030_3030_3030_3030, 0x0C0C_0C0C_0C0C_0C0C, 0x02);
    }
    if (shamt & 0x01) != 0 {
        x = shuffle64_stage(x, 0x4444_4444_4444_4444, 0x2222_2222_2222_2222, 0x01);
    }
    x
}

pub fn unshfl32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 15;
    if (shamt & 1) != 0 {
        x = shuffle32_stage(x, 0x4444_4444, 0x2222_2222, 1);
    }
    if (shamt & 2) != 0 {
        x = shuffle32_stage(x, 0x3030_3030, 0x0C0C_0C0C, 2);
    }
    if (shamt & 4) != 0 {
        x = shuffle32_stage(x, 0x0F00_0F00, 0x00F0_00F0, 4);
    }
    if (shamt & 8) != 0 {
        x = shuffle32_stage(x, 0x00FF_0000, 0x0000_FF00, 8);
    }
    x
}

pub fn unshfl64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x01) != 0 {
        x = shuffle64_stage(x, 0x4444_4444_4444_4444, 0x2222_2222_2222_2222, 0x01);
    }
    if (shamt & 0x02) != 0 {
        x = shuffle64_stage(x, 0x3030_3030_3030_3030, 0x0C0C_0C0C_0C0C_0C0C, 0x02);
    }
    if (shamt & 0x04) != 0 {
        x = shuffle64_stage(x, 0x0F00_0F00_0F00_0F00, 0x00F0_00F0_00F0_00F0, 0x04);
    }
    if (shamt & 0x08) != 0 {
        x = shuffle64_stage(x, 0x00FF_0000_00FF_0000, 0x0000_FF00_0000_FF00, 0x08);
    }
    if (shamt & 0x10) != 0 {
        x = shuffle64_stage(x, 0x0000_FFFF_0000_0000, 0x0000_0000_FFFF_0000, 0x10);
    }
    x
}

pub fn slo32(rs1: u32, rs2: u32) -> u32 {
    let shamt = rs2 & 31;
    !(!rs1 << shamt)
}

pub fn slo64(rs1: u64, rs2: u64) -> u64 {
    let shamt = rs2 & 63;
    !(!rs1 << shamt)
}

pub fn bfp32(rs1: u32, rs2: u32) -> u32 {
    let mut cfg: u32 = rs2 >> 16;
    if (cfg >> 30) == 2 {
        cfg >>= 16;
    }
    let mut len = (cfg >> 8) & 15;
    let off = cfg & 31;
    if len == 0 {
        len = 16
    }
    let mask = slo32(0, len) << off;
    let data = rs2 << off;
    (data & mask) | (rs1 & !mask)
}

pub fn bfp64(rs1: u64, rs2: u64) -> u64 {
    let mut cfg: u64 = rs2 >> 32;
    if (cfg >> 30) == 2 {
        cfg >>= 16;
    }
    let mut len = (cfg >> 8) & 31;
    let off = cfg & 63;
    if len == 0 {
        len = 32
    }
    let mask = slo64(0, len) << off;
    let data = rs2 << off;
    (data & mask) | (rs1 & !mask)
}

pub fn crc3232(x: u32, nbits: u32) -> u32 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0xEDB8_8320 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn crc3264(x: u64, nbits: u64) -> u64 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0xEDB8_8320 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn crc32c32(x: u32, nbits: u32) -> u32 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0x82F6_3B78 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn crc32c64(x: u64, nbits: u64) -> u64 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0x82F6_3B78 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn bmatflip(rs1: u64) -> u64 {
    let mut x = rs1;
    x = shfl64(x, 31);
    x = shfl64(x, 31);
    x = shfl64(x, 31);
    x
}

pub fn bmatxor(rs1: u64, rs2: u64) -> u64 {
    let rs2t = bmatflip(rs2);
    let mut u: [u8; 8] = [0u8; 8];
    let mut v: [u8; 8] = [0u8; 8];

    for i in 0..8 {
        u[i] = (rs1 >> (i * 8)) as u8;
        v[i] = (rs2t >> (i * 8)) as u8;
    }
    let mut x = 0;
    for i in 0..64 {
        if (u[i / 8] & v[i % 8]).count_ones() & 1 != 0 {
            x |= 1 << i;
        }
    }
    x
}

pub fn bmator(rs1: u64, rs2: u64) -> u64 {
    let rs2t = bmatflip(rs2);
    let mut u: [u8; 8] = [0u8; 8];
    let mut v: [u8; 8] = [0u8; 8];

    for i in 0..8 {
        u[i] = (rs1 >> (i * 8)) as u8;
        v[i] = (rs2t >> (i * 8)) as u8;
    }
    let mut x = 0;
    for i in 0..64 {
        if (u[i / 8] & v[i % 8]) != 0 {
            x |= 1 << i;
        }
    }
    x
}
