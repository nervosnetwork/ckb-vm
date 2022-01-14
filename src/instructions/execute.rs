use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, R4type, Register, Rtype, Stype, Utype, VItype, VVtype, VXtype,
};
use crate::memory::Memory;
use ckb_vm_definitions::{instructions as insts, registers::RA, VLEN};
pub use uintxx::{alu, Element, U1024, U128, U16, U2048, U256, U32, U512, U64, U8};

macro_rules! ld {
    ($inst:expr, $machine:expr, $size:expr) => {
        let i = Itype($inst);
        let rd = i.rd();
        let addr = $machine.registers()[i.rs1()].clone();
        let data = $machine.memory_mut().load_bytes(addr.to_u64(), $size)?;
        $machine
            .element_mut(rd, $size << 3, 0)
            .copy_from_slice(&data);
    };
}

macro_rules! sd {
    ($inst:expr, $machine:expr, $size:expr) => {
        let i = Itype($inst);
        let rd = i.rd();
        let addr = $machine.registers()[i.rs1()].clone();
        let data = $machine.element_ref(rd, $size << 3, 0).to_vec();
        $machine.memory_mut().store_bytes(addr.to_u64(), &data)?;
    };
}

macro_rules! vv_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::read($machine.element_ref(i.vs1(), sew, j));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vv_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        vv_loop_s!($inst, $machine, $body);
    };
}

macro_rules! vx_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vx_s($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vx_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vx_u($machine.registers()[i.rs1()].to_u64());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vi_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vi_s(i.immediate_s());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vi_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vi_u(i.immediate_u());
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vv_loop_s_cmp {
    ($inst:expr, $machine:expr, $cond:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        let mut out = [0u8; VLEN >> 3];
        out.copy_from_slice($machine.element_ref(i.vd(), VLEN as u64, 0));
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::read($machine.element_ref(i.vs1(), sew, j));
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                _ => return Err(Error::Unexpected),
            }
        }
        $machine
            .element_mut(i.vd(), VLEN as u64, 0)
            .copy_from_slice(&out);
    };
}

macro_rules! vv_loop_u_cmp {
    ($inst:expr, $machine:expr, $body:expr) => {
        vv_loop_s_cmp!($inst, $machine, $body);
    };
}

macro_rules! vx_loop_s_cmp {
    ($inst:expr, $machine:expr, $cond:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        let mut out = [0u8; VLEN >> 3];
        out.copy_from_slice($machine.element_ref(i.vd(), VLEN as u64, 0));
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vx_s($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                _ => return Err(Error::Unexpected),
            }
        }
        $machine
            .element_mut(i.vd(), VLEN as u64, 0)
            .copy_from_slice(&out);
    };
}

macro_rules! vx_loop_u_cmp {
    ($inst:expr, $machine:expr, $cond:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        let mut out = [0u8; VLEN >> 3];
        out.copy_from_slice($machine.element_ref(i.vd(), VLEN as u64, 0));
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vx_u($machine.registers()[i.rs1()].to_u64());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                _ => return Err(Error::Unexpected),
            }
        }
        $machine
            .element_mut(i.vd(), VLEN as u64, 0)
            .copy_from_slice(&out);
    };
}

macro_rules! vi_loop_s_cmp {
    ($inst:expr, $machine:expr, $cond:expr) => {
        let i = VItype($inst);
        let sew = $machine.vsew();
        let mut out = [0u8; VLEN >> 3];
        out.copy_from_slice($machine.element_ref(i.vd(), VLEN as u64, 0));
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vi_s(i.immediate_s());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                _ => return Err(Error::Unexpected),
            }
        }
        $machine
            .element_mut(i.vd(), VLEN as u64, 0)
            .copy_from_slice(&out);
    };
}

macro_rules! vi_loop_u_cmp {
    ($inst:expr, $machine:expr, $cond:expr) => {
        let i = VItype($inst);
        let sew = $machine.vsew();
        let mut out = [0u8; VLEN >> 3];
        out.copy_from_slice($machine.element_ref(i.vd(), VLEN as u64, 0));
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vi_u(i.immediate_u());
                    out[j / 8] = if $cond(b, a) {
                        out[j / 8] | (1 << (j % 8))
                    } else {
                        out[j / 8] & !(1 << (j % 8))
                    };
                }
                _ => return Err(Error::Unexpected),
            }
        }
        $machine
            .element_mut(i.vd(), VLEN as u64, 0)
            .copy_from_slice(&out);
    };
}

macro_rules! vv_loop_s_widen {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::read($machine.element_ref(i.vs1(), sew, j));
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vv_loop_u_widen {
    ($inst:expr, $machine:expr, $cond:expr) => {
        vv_loop_s_widen!($inst, $machine, $cond);
    };
}

macro_rules! vx_loop_s_widen {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vx_s($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vx_loop_u_widen {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U8::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U8::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                16 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U16::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                32 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U32::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                64 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U64::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                128 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U128::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                256 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U256::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                512 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U512::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                1024 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew, j));
                    let a = U1024::vx_u($machine.registers()[i.rs1()].to_u64());
                    let (lo, hi) = $body(b, a);
                    lo.save($machine.element_mut(i.vd(), sew, j * 2));
                    hi.save($machine.element_mut(i.vd(), sew, j * 2 + 1));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! wv_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a.wrapping_shl(U16::BITS / 2).wrapping_sra(U16::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a.wrapping_shl(U32::BITS / 2).wrapping_sra(U32::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a.wrapping_shl(U64::BITS / 2).wrapping_sra(U64::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a.wrapping_shl(U128::BITS / 2).wrapping_sra(U128::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a.wrapping_shl(U256::BITS / 2).wrapping_sra(U256::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a.wrapping_shl(U512::BITS / 2).wrapping_sra(U512::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a
                        .wrapping_shl(U1024::BITS / 2)
                        .wrapping_sra(U1024::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::read($machine.element_ref(i.vs1(), sew, j)));
                    let a = a
                        .wrapping_shl(U2048::BITS / 2)
                        .wrapping_sra(U2048::BITS / 2);
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! wv_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! wx_loop_s {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::vx_s($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! wx_loop_u {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a);
                    r.save($machine.element_mut(i.vd(), sew * 2, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! vv_loop_s_ext {
    ($inst:expr, $machine:expr, $size:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mut b = $machine.element_ref(i.vs2(), sew / $size, j).to_vec();
            if b.last().unwrap() > &127 {
                b.resize(sew as usize >> 3, 0xff);
            } else {
                b.resize(sew as usize >> 3, 0x00);
            }
            $machine.element_mut(i.vd(), sew, j).copy_from_slice(&b);
        }
    };
}

macro_rules! vv_loop_u_ext {
    ($inst:expr, $machine:expr, $size:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            let mut b = $machine.element_ref(i.vs2(), sew / $size, j).to_vec();
            b.resize(sew as usize >> 3, 0x00);
            $machine.element_mut(i.vd(), sew, j).copy_from_slice(&b);
        }
    };
}

macro_rules! wv_loop_u_nshift {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VVtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::read($machine.element_ref(i.vs1(), sew, j)));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! wx_loop_u_nshift {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VXtype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::vx_u($machine.registers()[i.rs1()].to_u64()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

macro_rules! wi_loop_u_nshift {
    ($inst:expr, $machine:expr, $body:expr) => {
        let i = VItype($inst);
        let sew = $machine.vsew();
        for j in 0..$machine.vl() as usize {
            match sew {
                8 => {
                    let b = U16::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U16::from(U8::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                16 => {
                    let b = U32::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U32::from(U16::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                32 => {
                    let b = U64::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U64::from(U32::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                64 => {
                    let b = U128::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U128::from(U64::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                128 => {
                    let b = U256::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U256::from(U128::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                256 => {
                    let b = U512::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U512::from(U256::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                512 => {
                    let b = U1024::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U1024::from(U512::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                1024 => {
                    let b = U2048::read($machine.element_ref(i.vs2(), sew * 2, j));
                    let a = U2048::from(U1024::vi_u(i.immediate_u()));
                    let r = $body(b, a).lo_zext();
                    r.save_lo($machine.element_mut(i.vd(), sew, j));
                }
                _ => return Err(Error::Unexpected),
            }
        }
    };
}

pub fn execute_instruction<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
) -> Result<(), Error> {
    let op = extract_opcode(inst);
    match op {
        insts::OP_SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_XOR => {
            let i = Rtype(inst);
            common::xor(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_OR => {
            let i = Rtype(inst);
            common::or(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_AND => {
            let i = Rtype(inst);
            common::and(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_SLL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SRL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() >> shift_value;
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value =
                machine.registers()[i.rs1()].zero_extend(&Mac::REG::from_u8(32)) >> shift_value;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SRA => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].signed_shr(&shift_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRAW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value = machine.registers()[i.rs1()]
                .sign_extend(&Mac::REG::from_u8(32))
                .signed_shr(&shift_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SLT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_LB => {
            let i = Itype(inst);
            common::lb(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LH => {
            let i = Itype(inst);
            common::lh(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LW => {
            let i = Itype(inst);
            common::lw(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LD => {
            let i = Itype(inst);
            common::ld(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LBU => {
            let i = Itype(inst);
            common::lbu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LHU => {
            let i = Itype(inst);
            common::lhu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LWU => {
            let i = Itype(inst);
            common::lwu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_XORI => {
            let i = Itype(inst);
            common::xori(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ORI => {
            let i = Itype(inst);
            common::ori(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_SLTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt_s(&imm_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTIU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt(&imm_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_JALR => {
            let i = Itype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            if machine.version() >= 1 {
                let mut next_pc = machine.registers()[i.rs1()]
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
                next_pc = next_pc & (!Mac::REG::one());
                update_register(machine, i.rd(), link);
                machine.update_pc(next_pc);
            } else {
                update_register(machine, i.rd(), link);
                let mut next_pc = machine.registers()[i.rs1()]
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
                next_pc = next_pc & (!Mac::REG::one());
                machine.update_pc(next_pc);
            }
        }
        insts::OP_SLLI => {
            let i = Itype(inst);
            common::slli(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SLLIW => {
            let i = Itype(inst);
            common::slliw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRLIW => {
            let i = Itype(inst);
            common::srliw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRAIW => {
            let i = Itype(inst);
            common::sraiw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SB => {
            let i = Stype(inst);
            common::sb(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SH => {
            let i = Stype(inst);
            common::sh(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SW => {
            let i = Stype(inst);
            common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SD => {
            let i = Stype(inst);
            common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_BEQ => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.eq(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BNE => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ne(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BLT => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.lt_s(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BGE => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ge_s(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BLTU => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.lt(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BGEU => {
            let i = Stype(inst);
            let pc = machine.pc();
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ge(&rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
        }
        insts::OP_AUIPC => {
            let i = Utype(inst);
            let value = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ECALL => {
            // The semantic of ECALL is determined by the hardware, which
            // is not part of the spec, hence here the implementation is
            // deferred to the machine. This way custom ECALLs might be
            // provided for different environments.
            machine.ecall()?;
        }
        insts::OP_EBREAK => {
            machine.ebreak()?;
        }
        insts::OP_FENCEI => {}
        insts::OP_FENCE => {}
        insts::OP_JAL => {
            let i = Utype(inst);
            common::jal(machine, i.rd(), i.immediate_s(), instruction_length(inst));
        }
        insts::OP_MUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_u8(32)));
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_MULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHSU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_DIVU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_REM => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_REMU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_u = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs2_value.overflowing_add(&rs1_u);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ANDN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() & !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_BCLR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BCLRI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BEXT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BEXTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BINV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BINVI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BSET => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BSETI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clmul(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clmulh(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clmulr(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLZ => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.clz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLZW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .clz()
                .overflowing_sub(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_CPOP => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.cpop();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CPOPW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.zero_extend(&Mac::REG::from_u8(32)).cpop();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CTZ => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.ctz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CTZW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = (rs1_value.clone() | Mac::REG::from_u64(0xffff_ffff_0000_0000)).ctz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAX => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge_s(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAXU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MIN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MINU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ORCB => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.orcb();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ORN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() | !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_REV8 => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.rev8();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.rol(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROLW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.rol(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_SEXTB => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let shift = &Mac::REG::from_u8(Mac::REG::BITS - 8);
            let value = rs1_value.signed_shl(shift).signed_shr(shift);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SEXTH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let shift = &Mac::REG::from_u8(Mac::REG::BITS - 16);
            let value = rs1_value.signed_shl(shift).signed_shr(shift);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH1ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH1ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH2ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH2ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH3ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH3ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLLIUW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = Mac::REG::from_u32(i.immediate());
            let rs1_u = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let shamt = rs2_value & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_u << shamt;
            update_register(machine, i.rd(), value);
        }
        insts::OP_XNOR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() ^ !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ZEXTH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.zero_extend(&Mac::REG::from_u8(16));
            update_register(machine, i.rd(), value);
        }
        insts::OP_WIDE_MUL => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULSU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIV => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div_signed(&rs2_value);
            let value_l = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIVU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div(&rs2_value);
            let value_l = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_FAR_JUMP_REL => {
            let i = Utype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let next_pc = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()))
                & (!Mac::REG::one());
            update_register(machine, RA, link);
            machine.update_pc(next_pc);
        }
        insts::OP_FAR_JUMP_ABS => {
            let i = Utype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let next_pc = Mac::REG::from_i32(i.immediate_s()) & (!Mac::REG::one());
            update_register(machine, RA, link);
            machine.update_pc(next_pc);
        }
        insts::OP_ADC => {
            let i = Rtype(inst);
            let rd_value = &machine.registers()[i.rd()];
            let rs1_value = &machine.registers()[i.rs1()];
            let r = rd_value.overflowing_add(&rs1_value);
            update_register(machine, i.rd(), r);
            let rd_value = &machine.registers()[i.rd()];
            let rs1_value = &machine.registers()[i.rs1()];
            let r = rd_value.lt(&rs1_value);
            update_register(machine, i.rs1(), r);
            let rd_value = &machine.registers()[i.rd()];
            let rs2_value = &machine.registers()[i.rs2()];
            let r = rd_value.overflowing_add(&rs2_value);
            update_register(machine, i.rd(), r);
            let rd_value = &machine.registers()[i.rd()];
            let rs2_value = &machine.registers()[i.rs2()];
            let r = rd_value.lt(&rs2_value);
            update_register(machine, i.rs2(), r);
            let rs1_value = machine.registers()[i.rs1()].clone();
            let rs2_value = machine.registers()[i.rs2()].clone();
            let r = rs1_value | rs2_value;
            update_register(machine, i.rs1(), r);
        }
        insts::OP_SBB => {
            let i = R4type(inst);
            let rd_value = &machine.registers()[i.rd()];
            let rs1_value = &machine.registers()[i.rs1()];
            let r = rd_value.overflowing_sub(&rs1_value);
            update_register(machine, i.rs1(), r);
            let rd_value = &machine.registers()[i.rd()];
            let rs1_value = &machine.registers()[i.rs1()];
            let r = rd_value.lt(&rs1_value);
            update_register(machine, i.rs3(), r);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let r = rs1_value.overflowing_sub(&rs2_value);
            update_register(machine, i.rd(), r);
            let rd_value = &machine.registers()[i.rd()];
            let rs1_value = &machine.registers()[i.rs1()];
            let r = rs1_value.lt(&rd_value);
            update_register(machine, i.rs2(), r);
            let rs2_value = machine.registers()[i.rs2()].clone();
            let rs3_value = machine.registers()[i.rs3()].clone();
            let r = rs2_value | rs3_value;
            update_register(machine, i.rs1(), r);
        }
        insts::OP_LD_SIGN_EXTENDED_32_CONSTANT => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
        }
        insts::OP_CUSTOM_LOAD_IMM => {
            let i = Utype(inst);
            let value = Mac::REG::from_i32(i.immediate_s());
            update_register(machine, i.rd(), value);
        }
        insts::OP_VSETVLI => {
            let i = Itype(inst);
            common::set_vl(
                machine,
                i.rd(),
                i.rs1(),
                machine.registers()[i.rs1()].to_u64(),
                i.immediate() as u64,
            )?;
        }
        insts::OP_VSETIVLI => {
            let i = Itype(inst);
            common::set_vl(machine, i.rd(), 33, i.rs1() as u64, i.immediate() as u64)?;
        }
        insts::OP_VSETVL => {
            let i = Rtype(inst);
            common::set_vl(
                machine,
                i.rd(),
                i.rs1(),
                machine.registers()[i.rs1()].to_u64(),
                machine.registers()[i.rs2()].to_u64(),
            )?;
        }
        insts::OP_VLM_V => {
            let size = (machine.vl() + 7) / 8;
            ld!(inst, machine, size);
        }
        insts::OP_VLE8_V => {
            let size = machine.vl() * 8 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE16_V => {
            let size = machine.vl() * 16 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE32_V => {
            let size = machine.vl() * 32 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE64_V => {
            let size = machine.vl() * 64 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE128_V => {
            let size = machine.vl() * 128 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE256_V => {
            let size = machine.vl() * 256 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE512_V => {
            let size = machine.vl() * 512 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VLE1024_V => {
            let size = machine.vl() * 1024 >> 3;
            ld!(inst, machine, size);
        }
        insts::OP_VSM_V => {
            let size = (machine.vl() + 7) / 8;
            sd!(inst, machine, size);
        }
        insts::OP_VSE8_V => {
            let size = machine.vl() * 8 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE16_V => {
            let size = machine.vl() * 16 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE32_V => {
            let size = machine.vl() * 32 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE64_V => {
            let size = machine.vl() * 64 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE128_V => {
            let size = machine.vl() * 128 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE256_V => {
            let size = machine.vl() * 256 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE512_V => {
            let size = machine.vl() * 512 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VSE1024_V => {
            let size = machine.vl() * 1024 >> 3;
            sd!(inst, machine, size);
        }
        insts::OP_VADD_VV => {
            vv_loop_s!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VADD_VX => {
            vx_loop_s!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VADD_VI => {
            vi_loop_s!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VSUB_VV => {
            vv_loop_s!(inst, machine, { Element::wrapping_sub });
        }
        insts::OP_VSUB_VX => {
            vx_loop_s!(inst, machine, { Element::wrapping_sub });
        }
        insts::OP_VRSUB_VX => {
            vx_loop_s!(inst, machine, { Element::wrapping_rsub });
        }
        insts::OP_VRSUB_VI => {
            vi_loop_s!(inst, machine, { Element::wrapping_rsub });
        }
        insts::OP_VMUL_VV => {
            vv_loop_s!(inst, machine, { Element::wrapping_mul });
        }
        insts::OP_VMUL_VX => {
            vx_loop_s!(inst, machine, { Element::wrapping_mul });
        }
        insts::OP_VMULH_VV => {
            vv_loop_s!(inst, machine, { alu::mulh });
        }
        insts::OP_VMULH_VX => {
            vx_loop_s!(inst, machine, { alu::mulh });
        }
        insts::OP_VMULHU_VV => {
            vv_loop_u!(inst, machine, { alu::mulhu });
        }
        insts::OP_VMULHU_VX => {
            vx_loop_u!(inst, machine, { alu::mulhu });
        }
        insts::OP_VMULHSU_VV => {
            vv_loop_u!(inst, machine, { alu::mulhsu });
        }
        insts::OP_VMULHSU_VX => {
            vx_loop_u!(inst, machine, { alu::mulhsu });
        }
        insts::OP_VDIVU_VV => {
            vv_loop_u!(inst, machine, { Element::wrapping_div });
        }
        insts::OP_VDIVU_VX => {
            vx_loop_u!(inst, machine, { Element::wrapping_div });
        }
        insts::OP_VDIV_VV => {
            vv_loop_s!(inst, machine, { Element::wrapping_div_s });
        }
        insts::OP_VDIV_VX => {
            vx_loop_s!(inst, machine, { Element::wrapping_div_s });
        }
        insts::OP_VREMU_VV => {
            vv_loop_u!(inst, machine, { Element::wrapping_rem });
        }
        insts::OP_VREMU_VX => {
            vx_loop_u!(inst, machine, { Element::wrapping_rem });
        }
        insts::OP_VREM_VV => {
            vv_loop_s!(inst, machine, { Element::wrapping_rem_s });
        }
        insts::OP_VREM_VX => {
            vx_loop_s!(inst, machine, { Element::wrapping_rem_s });
        }
        insts::OP_VSLL_VV => {
            vv_loop_u!(inst, machine, { Element::wrapping_shl_e });
        }
        insts::OP_VSLL_VX => {
            vx_loop_u!(inst, machine, { Element::wrapping_shl_e });
        }
        insts::OP_VSLL_VI => {
            vi_loop_u!(inst, machine, { Element::wrapping_shl_e });
        }
        insts::OP_VSRL_VV => {
            vv_loop_u!(inst, machine, { Element::wrapping_shr_e });
        }
        insts::OP_VSRL_VX => {
            vx_loop_u!(inst, machine, { Element::wrapping_shr_e });
        }
        insts::OP_VSRL_VI => {
            vi_loop_u!(inst, machine, { Element::wrapping_shr_e });
        }
        insts::OP_VSRA_VV => {
            vv_loop_u!(inst, machine, { Element::wrapping_sra_e });
        }
        insts::OP_VSRA_VX => {
            vx_loop_u!(inst, machine, { Element::wrapping_sra_e });
        }
        insts::OP_VSRA_VI => {
            vi_loop_u!(inst, machine, { Element::wrapping_sra_e });
        }
        insts::OP_VMSEQ_VV => {
            vv_loop_s_cmp!(inst, machine, { alu::seq });
        }
        insts::OP_VMSEQ_VX => {
            vx_loop_s_cmp!(inst, machine, { alu::seq });
        }
        insts::OP_VMSEQ_VI => {
            vi_loop_s_cmp!(inst, machine, { alu::seq });
        }
        insts::OP_VMSNE_VV => {
            vv_loop_s_cmp!(inst, machine, { alu::sne });
        }
        insts::OP_VMSNE_VX => {
            vx_loop_s_cmp!(inst, machine, { alu::sne });
        }
        insts::OP_VMSNE_VI => {
            vi_loop_s_cmp!(inst, machine, { alu::sne });
        }
        insts::OP_VMSLTU_VV => {
            vv_loop_u_cmp!(inst, machine, { alu::sltu });
        }
        insts::OP_VMSLTU_VX => {
            vx_loop_u_cmp!(inst, machine, { alu::sltu });
        }
        insts::OP_VMSLT_VV => {
            vv_loop_s_cmp!(inst, machine, { alu::slt });
        }
        insts::OP_VMSLT_VX => {
            vx_loop_s_cmp!(inst, machine, { alu::slt });
        }
        insts::OP_VMSLEU_VV => {
            vv_loop_u_cmp!(inst, machine, { alu::sleu });
        }
        insts::OP_VMSLEU_VX => {
            vx_loop_u_cmp!(inst, machine, { alu::sleu });
        }
        insts::OP_VMSLEU_VI => {
            vi_loop_u_cmp!(inst, machine, { alu::sleu });
        }
        insts::OP_VMSLE_VV => {
            vv_loop_s_cmp!(inst, machine, { alu::sle });
        }
        insts::OP_VMSLE_VX => {
            vx_loop_s_cmp!(inst, machine, { alu::sle });
        }
        insts::OP_VMSLE_VI => {
            vi_loop_s_cmp!(inst, machine, { alu::sle });
        }
        insts::OP_VMSGTU_VX => {
            vx_loop_u_cmp!(inst, machine, { alu::sgtu });
        }
        insts::OP_VMSGTU_VI => {
            vi_loop_u_cmp!(inst, machine, { alu::sgtu });
        }
        insts::OP_VMSGT_VX => {
            vx_loop_s_cmp!(inst, machine, { alu::sgt });
        }
        insts::OP_VMSGT_VI => {
            vi_loop_s_cmp!(inst, machine, { alu::sgt });
        }
        insts::OP_VMAXU_VV => {
            vv_loop_u!(inst, machine, { alu::maxu });
        }
        insts::OP_VMAXU_VX => {
            vx_loop_u!(inst, machine, { alu::maxu });
        }
        insts::OP_VMAX_VV => {
            vv_loop_s!(inst, machine, { alu::max });
        }
        insts::OP_VMAX_VX => {
            vx_loop_s!(inst, machine, { alu::max });
        }
        insts::OP_VMINU_VV => {
            vv_loop_u!(inst, machine, { alu::minu });
        }
        insts::OP_VMINU_VX => {
            vx_loop_u!(inst, machine, { alu::minu });
        }
        insts::OP_VMIN_VV => {
            vv_loop_s!(inst, machine, { alu::min });
        }
        insts::OP_VMIN_VX => {
            vx_loop_s!(inst, machine, { alu::min });
        }
        insts::OP_VAND_VV => {
            vv_loop_s!(inst, machine, { alu::and });
        }
        insts::OP_VOR_VV => {
            vv_loop_s!(inst, machine, { alu::or });
        }
        insts::OP_VXOR_VV => {
            vv_loop_s!(inst, machine, { alu::xor });
        }
        insts::OP_VAND_VX => {
            vx_loop_s!(inst, machine, { alu::and });
        }
        insts::OP_VOR_VX => {
            vx_loop_s!(inst, machine, { alu::or });
        }
        insts::OP_VXOR_VX => {
            vx_loop_s!(inst, machine, { alu::xor });
        }
        insts::OP_VAND_VI => {
            vi_loop_s!(inst, machine, { alu::and });
        }
        insts::OP_VOR_VI => {
            vi_loop_s!(inst, machine, { alu::or });
        }
        insts::OP_VXOR_VI => {
            vi_loop_s!(inst, machine, { alu::xor });
        }
        insts::OP_VMV1R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 1, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 1, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VMV2R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 2, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 2, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VMV4R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 4, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 4, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VMV8R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 8, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 8, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VSADDU_VV => {
            vv_loop_u!(inst, machine, { alu::saddu });
        }
        insts::OP_VSADDU_VX => {
            vx_loop_u!(inst, machine, { alu::saddu });
        }
        insts::OP_VSADDU_VI => {
            vi_loop_u!(inst, machine, { alu::saddu });
        }
        insts::OP_VSADD_VV => {
            vv_loop_s!(inst, machine, { alu::sadd });
        }
        insts::OP_VSADD_VX => {
            vx_loop_s!(inst, machine, { alu::sadd });
        }
        insts::OP_VSADD_VI => {
            vi_loop_s!(inst, machine, { alu::sadd });
        }
        insts::OP_VSSUBU_VV => {
            vv_loop_u!(inst, machine, { alu::ssubu });
        }
        insts::OP_VSSUBU_VX => {
            vx_loop_u!(inst, machine, { alu::ssubu });
        }
        insts::OP_VSSUB_VV => {
            vv_loop_s!(inst, machine, { alu::ssub });
        }
        insts::OP_VSSUB_VX => {
            vx_loop_s!(inst, machine, { alu::ssub });
        }
        insts::OP_VWADDU_VV => {
            vv_loop_u_widen!(inst, machine, { Element::widening_add });
        }
        insts::OP_VWADDU_VX => {
            vx_loop_u_widen!(inst, machine, { Element::widening_add });
        }
        insts::OP_VWADDU_WV => {
            wv_loop_u!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VWADDU_WX => {
            wx_loop_u!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VWADD_WX => {
            wx_loop_s!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VWADD_VV => {
            vv_loop_s_widen!(inst, machine, { Element::widening_add_s });
        }
        insts::OP_VWADD_VX => {
            vx_loop_s_widen!(inst, machine, { Element::widening_add_s });
        }
        insts::OP_VWADD_WV => {
            wv_loop_s!(inst, machine, { Element::wrapping_add });
        }
        insts::OP_VWSUBU_VV => {
            vv_loop_u_widen!(inst, machine, { Element::widening_sub });
        }
        insts::OP_VWSUBU_WV => {
            wv_loop_u!(inst, machine, { Element::wrapping_sub });
        }
        insts::OP_VWSUBU_VX => {
            vx_loop_u_widen!(inst, machine, { Element::widening_sub });
        }
        insts::OP_VWSUB_VV => {
            vv_loop_s_widen!(inst, machine, { Element::widening_sub_s });
        }
        insts::OP_VWSUB_VX => {
            vx_loop_s_widen!(inst, machine, { Element::widening_sub_s });
        }
        insts::OP_VWSUB_WV => {
            wv_loop_s!(inst, machine, { Element::wrapping_sub });
        }
        insts::OP_VWSUBU_WX => {
            wx_loop_u!(inst, machine, { Element::wrapping_sub });
        }
        insts::OP_VWSUB_WX => {
            wx_loop_s!(inst, machine, { Element::wrapping_sub });
        }
        insts::OP_VWMULU_VV => {
            vv_loop_u_widen!(inst, machine, { Element::widening_mul });
        }
        insts::OP_VWMULU_VX => {
            vx_loop_u_widen!(inst, machine, { Element::widening_mul });
        }
        insts::OP_VWMULSU_VV => {
            vv_loop_u_widen!(inst, machine, { Element::widening_mul_su });
        }
        insts::OP_VWMULSU_VX => {
            vx_loop_u_widen!(inst, machine, { Element::widening_mul_su });
        }
        insts::OP_VWMUL_VV => {
            vv_loop_s_widen!(inst, machine, { Element::widening_mul_s });
        }
        insts::OP_VWMUL_VX => {
            vx_loop_s_widen!(inst, machine, { Element::widening_mul_s });
        }
        insts::OP_VAADD_VV => {
            vv_loop_s!(inst, machine, { Element::average_add_s });
        }
        insts::OP_VAADD_VX => {
            vx_loop_s!(inst, machine, { Element::average_add_s });
        }
        insts::OP_VAADDU_VV => {
            vv_loop_u!(inst, machine, { Element::average_add });
        }
        insts::OP_VAADDU_VX => {
            vx_loop_u!(inst, machine, { Element::average_add });
        }
        insts::OP_VASUB_VV => {
            vv_loop_s!(inst, machine, { Element::average_sub_s });
        }
        insts::OP_VASUB_VX => {
            vx_loop_s!(inst, machine, { Element::average_sub_s });
        }
        insts::OP_VASUBU_VV => {
            vv_loop_u!(inst, machine, { Element::average_sub });
        }
        insts::OP_VASUBU_VX => {
            vx_loop_u!(inst, machine, { Element::average_sub });
        }
        insts::OP_VMV_VV => {
            vv_loop_s!(inst, machine, { alu::mv });
        }
        insts::OP_VMV_VX => {
            vx_loop_s!(inst, machine, { alu::mv });
        }
        insts::OP_VMV_VI => {
            vi_loop_s!(inst, machine, { alu::mv });
        }
        insts::OP_VZEXT_VF2 => {
            vv_loop_u_ext!(inst, machine, 2);
        }
        insts::OP_VZEXT_VF4 => {
            vv_loop_u_ext!(inst, machine, 4);
        }
        insts::OP_VZEXT_VF8 => {
            vv_loop_u_ext!(inst, machine, 8);
        }
        insts::OP_VSEXT_VF2 => {
            vv_loop_s_ext!(inst, machine, 2);
        }
        insts::OP_VSEXT_VF4 => {
            vv_loop_s_ext!(inst, machine, 4);
        }
        insts::OP_VSEXT_VF8 => {
            vv_loop_s_ext!(inst, machine, 8);
        }
        insts::OP_VNSRL_WV => {
            wv_loop_u_nshift!(inst, machine, { Element::wrapping_shr_e });
        }
        insts::OP_VNSRL_WX => {
            wx_loop_u_nshift!(inst, machine, { Element::wrapping_shr_e });
        }
        insts::OP_VNSRL_WI => {
            wi_loop_u_nshift!(inst, machine, { Element::wrapping_shr_e });
        }
        insts::OP_VNSRA_WV => {
            wv_loop_u_nshift!(inst, machine, { Element::wrapping_sra_e });
        }
        insts::OP_VNSRA_WX => {
            wx_loop_u_nshift!(inst, machine, { Element::wrapping_sra_e });
        }
        insts::OP_VNSRA_WI => {
            wi_loop_u_nshift!(inst, machine, { Element::wrapping_sra_e });
        }
        insts::OP_VFIRST_M => {
            let i = Rtype(inst);
            let m = U2048::read(machine.element_ref(i.rs2(), VLEN as u64, 0));
            let r = m.trailing_zeros();
            if r == 2048 {
                update_register(machine, i.rd(), Mac::REG::from_u64(0xffff_ffff_ffff_ffff));
            } else {
                update_register(machine, i.rd(), Mac::REG::from_u32(r));
            }
        }
        _ => return Err(Error::InvalidOp(op)),
    };
    Ok(())
}

pub fn execute<Mac: Machine>(inst: Instruction, machine: &mut Mac) -> Result<(), Error> {
    let instruction_size = instruction_length(inst);
    let next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_u8(instruction_size));
    machine.update_pc(next_pc);
    let r = execute_instruction(inst, machine);
    machine.commit_pc();
    r
}
