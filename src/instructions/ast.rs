use crate::Register;
use std::fmt::{self, Display};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum ActionOp1 {
    Not,
    LogicalNot,
    Clz,
    Ctz,
    Pcnt,
}

#[derive(Debug, Clone, Copy)]
pub enum ActionOp2 {
    Add,
    Sub,
    Mul,
    Mulhsu,
    Bitand,
    Bitor,
    Bitxor,
    Shl,
    Eq,
    Rol,
    Ror,
    Slo,
    Sro,
}

#[derive(Debug, Clone, Copy)]
pub enum SignActionOp2 {
    Mulh,
    Div,
    Rem,
    Shr,
    Lt,
    Extend,
}

#[derive(Debug, Clone, Copy)]
pub enum ActionOp3 {
    Fsl,
    Fsr,
}

#[derive(Debug, Clone)]
pub enum Value {
    Imm(u64),
    Register(usize),
    Op1(ActionOp1, Rc<Value>),
    Op2(ActionOp2, Rc<Value>, Rc<Value>),
    SignOp2(SignActionOp2, Rc<Value>, Rc<Value>, bool),
    Op3(ActionOp3, Rc<Value>, Rc<Value>, Rc<Value>),
    Cond(Rc<Value>, Rc<Value>, Rc<Value>),
    Load(Rc<Value>, u8),
}

impl Default for Value {
    fn default() -> Value {
        Value::zero()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Value {
        if let Value::Imm(imm) = self {
            return Value::Imm(!imm);
        }
        Value::Op1(ActionOp1::Not, Rc::new(self))
    }
}

impl BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(imm1 & imm2);
        }
        Value::Op2(ActionOp2::Bitand, Rc::new(self), Rc::new(rhs))
    }
}

impl BitOr for Value {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(imm1 | imm2);
        }
        Value::Op2(ActionOp2::Bitor, Rc::new(self), Rc::new(rhs))
    }
}

impl BitXor for Value {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(imm1 ^ imm2);
        }
        Value::Op2(ActionOp2::Bitxor, Rc::new(self), Rc::new(rhs))
    }
}

impl Shl<Value> for Value {
    type Output = Self;

    fn shl(self, rhs: Self) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            // By default immediates are unsigned
            return Value::Imm(imm1 << imm2);
        }
        Value::Op2(ActionOp2::Shl, Rc::new(self), Rc::new(rhs))
    }
}

impl Shr<Value> for Value {
    type Output = Self;

    fn shr(self, rhs: Self) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            // By default immediates are unsigned
            return Value::Imm(imm1 >> imm2);
        }
        Value::SignOp2(SignActionOp2::Shr, Rc::new(self), Rc::new(rhs), false)
    }
}

impl Register for Value {
    // For now we only support JIT on 64 bit RISC-V machine
    const BITS: u8 = 64;
    const SHIFT_MASK: u8 = 0x3F;

    fn zero() -> Value {
        Value::Imm(0)
    }

    fn one() -> Value {
        Value::Imm(1)
    }

    fn min_value() -> Value {
        Value::Imm(u64::min_value())
    }

    fn max_value() -> Value {
        Value::Imm(u64::max_value())
    }

    fn eq(&self, other: &Value) -> Value {
        Value::Op2(ActionOp2::Eq, Rc::new(self.clone()), Rc::new(other.clone()))
    }

    fn lt(&self, other: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Lt,
            Rc::new(self.clone()),
            Rc::new(other.clone()),
            false,
        )
    }

    fn lt_s(&self, other: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Lt,
            Rc::new(self.clone()),
            Rc::new(other.clone()),
            true,
        )
    }

    fn logical_not(&self) -> Value {
        Value::Op1(ActionOp1::LogicalNot, Rc::new(self.clone()))
    }

    fn cond(&self, true_value: &Value, false_value: &Value) -> Value {
        Value::Cond(
            Rc::new(self.clone()),
            Rc::new(true_value.clone()),
            Rc::new(false_value.clone()),
        )
    }

    fn overflowing_add(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (self, rhs) {
            let imm = (*imm1).overflowing_add(*imm2).0;
            return Value::Imm(imm);
        }
        Value::Op2(ActionOp2::Add, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn overflowing_sub(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (self, rhs) {
            let imm = (*imm1).overflowing_sub(*imm2).0;
            return Value::Imm(imm);
        }
        Value::Op2(ActionOp2::Sub, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn overflowing_mul(&self, rhs: &Value) -> Value {
        Value::Op2(ActionOp2::Mul, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn overflowing_div(&self, rhs: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Div,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            false,
        )
    }

    fn overflowing_rem(&self, rhs: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Rem,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            false,
        )
    }

    fn overflowing_div_signed(&self, rhs: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Div,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            true,
        )
    }

    fn overflowing_rem_signed(&self, rhs: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Rem,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            true,
        )
    }

    fn overflowing_mul_high_signed(&self, rhs: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Mulh,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            true,
        )
    }

    fn overflowing_mul_high_unsigned(&self, rhs: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Mulh,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            false,
        )
    }

    fn overflowing_mul_high_signed_unsigned(&self, rhs: &Value) -> Value {
        Value::Op2(
            ActionOp2::Mulhsu,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
        )
    }

    fn clz(&self) -> Value {
        Value::Op1(ActionOp1::Clz, Rc::new(self.clone()))
    }

    fn ctz(&self) -> Value {
        Value::Op1(ActionOp1::Ctz, Rc::new(self.clone()))
    }

    fn pcnt(&self) -> Value {
        Value::Op1(ActionOp1::Pcnt, Rc::new(self.clone()))
    }

    fn rol(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(imm1.rotate_left(*imm2 as u32));
        }
        Value::Op2(ActionOp2::Rol, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn ror(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(imm1.rotate_right(*imm2 as u32));
        }
        Value::Op2(ActionOp2::Ror, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn slo(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(!((!*imm1).shl(*imm2 as u32)));
        }
        Value::Op2(ActionOp2::Slo, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn sro(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (&self, &rhs) {
            return Value::Imm(!((!*imm1).shr(*imm2 as u32)));
        }
        Value::Op2(ActionOp2::Sro, Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn fsl(&self, rhs: &Value, shift: &Value) -> Value {
        Value::Op3(
            ActionOp3::Fsl,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            Rc::new(shift.clone()),
        )
    }

    fn fsr(&self, rhs: &Value, shift: &Value) -> Value {
        Value::Op3(
            ActionOp3::Fsr,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            Rc::new(shift.clone()),
        )
    }

    fn signed_shl(&self, rhs: &Value) -> Value {
        // Signed shl and unsigned shl are the same thing
        self.clone().shl(rhs.clone())
    }

    fn signed_shr(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (self, rhs) {
            // By default immediates are unsigned
            return Value::Imm(((*imm1 as i64) >> imm2) as u64);
        }
        Value::SignOp2(
            SignActionOp2::Shr,
            Rc::new(self.clone()),
            Rc::new(rhs.clone()),
            true,
        )
    }

    fn zero_extend(&self, start_bit: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Extend,
            Rc::new(self.clone()),
            Rc::new(start_bit.clone()),
            false,
        )
    }

    fn sign_extend(&self, start_bit: &Value) -> Value {
        Value::SignOp2(
            SignActionOp2::Extend,
            Rc::new(self.clone()),
            Rc::new(start_bit.clone()),
            true,
        )
    }

    fn to_i8(&self) -> i8 {
        0
    }

    fn to_i16(&self) -> i16 {
        0
    }

    fn to_i32(&self) -> i32 {
        0
    }

    fn to_i64(&self) -> i64 {
        0
    }

    fn to_u8(&self) -> u8 {
        0
    }

    fn to_u16(&self) -> u16 {
        0
    }

    fn to_u32(&self) -> u32 {
        0
    }

    fn to_u64(&self) -> u64 {
        0
    }

    fn from_i8(v: i8) -> Value {
        Value::Imm(i64::from(v) as u64)
    }

    fn from_i16(v: i16) -> Value {
        Value::Imm(i64::from(v) as u64)
    }

    fn from_i32(v: i32) -> Value {
        Value::Imm(i64::from(v) as u64)
    }

    fn from_i64(v: i64) -> Value {
        Value::Imm(v as u64)
    }

    fn from_u8(v: u8) -> Value {
        Value::Imm(u64::from(v))
    }

    fn from_u16(v: u16) -> Value {
        Value::Imm(u64::from(v))
    }

    fn from_u32(v: u32) -> Value {
        Value::Imm(u64::from(v))
    }

    fn from_u64(v: u64) -> Value {
        Value::Imm(v)
    }
}
