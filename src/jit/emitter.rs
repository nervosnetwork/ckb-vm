use super::{
    machine::{MemorySize, Write, REGISTER_PC},
    value::{ActionOp1, ActionOp2, SignActionOp2, Value},
};
use crate::{registers::A0, Error, RISCV_GENERAL_REGISTER_NUMBER};
use libc::{c_int, c_void, size_t, uint32_t};
// use std::rc::Rc;

// This is a C struct type
#[repr(C)]
struct AsmContext {
    _private: [u8; 0],
}

#[repr(C)]
union AsmValueContent {
    reg: u32,
    i: u64,
}

const ASM_TAG_REGISTER: u32 = 0x1;
const ASM_TAG_IMMEDIATE: u32 = 0x2;

#[repr(C)]
struct AsmValue {
    tag: u32,
    value: AsmValueContent,
}

extern "C" {
    fn asm_new() -> *mut AsmContext;
    fn asm_finalize(c: *mut AsmContext);
    fn asm_setup(c: *mut AsmContext) -> c_int;
    fn asm_emit_prologue(c: *mut AsmContext) -> c_int;
    fn asm_emit_epilogue(c: *mut AsmContext) -> c_int;
    fn asm_link(c: *mut AsmContext, szp: *mut size_t) -> c_int;
    fn asm_encode(c: *mut AsmContext, buffer: *mut c_void) -> c_int;

    fn asm_mov(c: *mut AsmContext, target: uint32_t, value: AsmValue) -> c_int;
    fn asm_add(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_sub(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_mul(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_mulh(
        c: *mut AsmContext,
        target: uint32_t,
        a: AsmValue,
        b: AsmValue,
        is_signed: c_int,
    ) -> c_int;
    fn asm_mulhsu(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_div(
        c: *mut AsmContext,
        target: uint32_t,
        a: AsmValue,
        b: AsmValue,
        is_signed: c_int,
    ) -> c_int;
    fn asm_rem(
        c: *mut AsmContext,
        target: uint32_t,
        a: AsmValue,
        b: AsmValue,
        is_signed: c_int,
    ) -> c_int;
    fn asm_and(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_or(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_not(c: *mut AsmContext, target: uint32_t, a: AsmValue, is_signed: c_int) -> c_int;
    fn asm_xor(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_shl(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_shr(
        c: *mut AsmContext,
        target: uint32_t,
        a: AsmValue,
        b: AsmValue,
        is_signed: c_int,
    ) -> c_int;
    fn asm_eq(c: *mut AsmContext, target: uint32_t, a: AsmValue, b: AsmValue) -> c_int;
    fn asm_lt(
        c: *mut AsmContext,
        target: uint32_t,
        a: AsmValue,
        b: AsmValue,
        is_signed: c_int,
    ) -> c_int;
    fn asm_cond(
        c: *mut AsmContext,
        target: uint32_t,
        condition: AsmValue,
        true_value: AsmValue,
        false_value: AsmValue,
    ) -> c_int;
    fn asm_extend(
        c: *mut AsmContext,
        target: uint32_t,
        a: AsmValue,
        b: AsmValue,
        is_signed: c_int,
    ) -> c_int;

    fn asm_push(c: *mut AsmContext, reg: uint32_t) -> c_int;
    fn asm_pop(c: *mut AsmContext, reg: uint32_t) -> c_int;

    fn asm_memory_read(
        c: *mut AsmContext,
        target: uint32_t,
        addr: AsmValue,
        size: uint32_t,
    ) -> c_int;
    fn asm_memory_write(c: *mut AsmContext, addr: AsmValue, v: AsmValue, size: uint32_t) -> c_int;
}

fn immediate_to_asm_value(imm: u64) -> AsmValue {
    AsmValue {
        tag: ASM_TAG_IMMEDIATE,
        value: AsmValueContent { i: imm },
    }
}

fn register_to_asm_value(register: usize) -> AsmValue {
    AsmValue {
        tag: ASM_TAG_REGISTER,
        value: AsmValueContent {
            reg: register as u32,
        },
    }
}

fn check_asm_result(result: c_int) -> Result<(), Error> {
    if result == 0 {
        Ok(())
    } else {
        Err(Error::Dynasm(result))
    }
}

struct RegisterAllocator {
    current: usize,
    avoids: Vec<usize>,
}

impl RegisterAllocator {
    pub fn new_with_avoids(avoids: &[usize]) -> Self {
        Self {
            current: A0,
            avoids: avoids.to_vec(),
        }
    }

    pub fn new_with_values(values: &[&Value]) -> Self {
        let mut avoids = vec![];
        for value in values {
            if let Value::Register(r) = value {
                avoids.push(*r);
            }
        }
        Self::new_with_avoids(&avoids)
    }

    pub fn next(&mut self) -> Result<usize, Error> {
        while self.current < RISCV_GENERAL_REGISTER_NUMBER && self.avoids.contains(&self.current) {
            self.current += 1;
        }
        if self.current < RISCV_GENERAL_REGISTER_NUMBER {
            let value = self.current;
            self.current += 1;
            Ok(value)
        } else {
            Err(Error::OutOfBound)
        }
    }
}

pub struct Emitter {
    asm: *mut AsmContext,
}

impl Drop for Emitter {
    fn drop(&mut self) {
        unsafe {
            asm_finalize(self.asm);
        }
    }
}

impl Emitter {
    pub fn new() -> Result<Emitter, Error> {
        let asm = unsafe { asm_new() };
        if asm.is_null() {
            Err(Error::Dynasm(-1))
        } else {
            Ok(Emitter { asm })
        }
    }

    pub fn setup(&mut self) -> Result<(), Error> {
        let result = unsafe { asm_setup(self.asm) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        let result = unsafe { asm_emit_prologue(self.asm) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(())
    }

    pub fn link(&mut self) -> Result<usize, Error> {
        let result = unsafe { asm_emit_epilogue(self.asm) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        let mut buffer_size: usize = 0;
        let result = unsafe { asm_link(self.asm, &mut buffer_size) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(buffer_size)
    }

    pub fn encode(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let result = unsafe { asm_encode(self.asm, buffer.as_mut_ptr() as *mut c_void) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(())
    }

    pub fn emit_write(&mut self, write: &Write) -> Result<(), Error> {
        match write {
            Write::Memory {
                address,
                size,
                value,
            } => self.emit_memory_write(address, *size, value),
            Write::Register { index, value } => self.emit(*index, value),
            Write::Pc { value } => self.emit(REGISTER_PC, value),
        }
    }

    pub fn emit_memory_write(
        &mut self,
        address: &Value,
        size: MemorySize,
        value: &Value,
    ) -> Result<(), Error> {
        let mut allocator = RegisterAllocator::new_with_values(&[address, value]);
        let address_register = allocator.next()?;
        let value_register = allocator.next()?;
        let (address, address_saved) = self.emit_intermediate(address_register, address)?;
        let (value, value_saved) = self.emit_intermediate(value_register, value)?;
        check_asm_result(unsafe { asm_memory_write(self.asm, address, value, size as u32) })?;
        if value_saved {
            check_asm_result(unsafe { asm_pop(self.asm, value_register as u32) })?;
        }
        if address_saved {
            check_asm_result(unsafe { asm_pop(self.asm, address_register as u32) })?;
        }
        Ok(())
    }

    pub fn emit(&mut self, target_register: usize, value: &Value) -> Result<(), Error> {
        match value {
            Value::Register(reg) => {
                if *reg != target_register {
                    check_asm_result(unsafe {
                        asm_mov(
                            self.asm,
                            target_register as u32,
                            register_to_asm_value(*reg),
                        )
                    })
                } else {
                    Ok(())
                }
            }
            Value::Imm(imm) => check_asm_result(unsafe {
                asm_mov(
                    self.asm,
                    target_register as u32,
                    immediate_to_asm_value(*imm),
                )
            }),
            Value::Op1(op, a) => {
                let mut allocator = RegisterAllocator::new_with_avoids(&[target_register]);
                let a_register = allocator.next()?;
                let (a, a_saved) = self.emit_intermediate(a_register, a)?;
                let result = match op {
                    ActionOp1::Not => unsafe { asm_not(self.asm, target_register as u32, a, 0) },
                    ActionOp1::LogicalNot => unsafe {
                        asm_not(self.asm, target_register as u32, a, 1)
                    },
                };
                check_asm_result(result)?;
                if a_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, a_register as u32) })?;
                }
                Ok(())
            }
            Value::Op2(op, a, b) => {
                let mut allocator = RegisterAllocator::new_with_avoids(&[target_register]);
                let a_register = allocator.next()?;
                let b_register = allocator.next()?;
                let (a, a_saved) = self.emit_intermediate(a_register, a)?;
                let (b, b_saved) = self.emit_intermediate(b_register, b)?;
                let result = match op {
                    ActionOp2::Add => unsafe { asm_add(self.asm, target_register as u32, a, b) },
                    ActionOp2::Sub => unsafe { asm_sub(self.asm, target_register as u32, a, b) },
                    ActionOp2::Mul => unsafe { asm_mul(self.asm, target_register as u32, a, b) },
                    ActionOp2::Mulhsu => unsafe {
                        asm_mulhsu(self.asm, target_register as u32, a, b)
                    },
                    ActionOp2::Bitand => unsafe { asm_and(self.asm, target_register as u32, a, b) },
                    ActionOp2::Bitor => unsafe { asm_or(self.asm, target_register as u32, a, b) },
                    ActionOp2::Bitxor => unsafe { asm_xor(self.asm, target_register as u32, a, b) },
                    ActionOp2::Shl => unsafe { asm_shl(self.asm, target_register as u32, a, b) },
                    ActionOp2::Eq => unsafe { asm_eq(self.asm, target_register as u32, a, b) },
                };
                check_asm_result(result)?;
                if b_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, b_register as u32) })?;
                }
                if a_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, a_register as u32) })?;
                }
                Ok(())
            }
            Value::SignOp2(op, a, b, signed) => {
                let mut allocator = RegisterAllocator::new_with_avoids(&[target_register]);
                let a_register = allocator.next()?;
                let b_register = allocator.next()?;
                let (a, a_saved) = self.emit_intermediate(a_register, a)?;
                let (b, b_saved) = self.emit_intermediate(b_register, b)?;
                let signed: c_int = (*signed).into();
                let result = match op {
                    SignActionOp2::Mulh => unsafe {
                        asm_mulh(self.asm, target_register as u32, a, b, signed)
                    },
                    SignActionOp2::Div => unsafe {
                        asm_div(self.asm, target_register as u32, a, b, signed)
                    },
                    SignActionOp2::Rem => unsafe {
                        asm_rem(self.asm, target_register as u32, a, b, signed)
                    },
                    SignActionOp2::Shr => unsafe {
                        asm_shr(self.asm, target_register as u32, a, b, signed)
                    },
                    SignActionOp2::Lt => unsafe {
                        asm_lt(self.asm, target_register as u32, a, b, signed)
                    },
                    SignActionOp2::Extend => unsafe {
                        asm_extend(self.asm, target_register as u32, a, b, signed)
                    },
                };
                check_asm_result(result)?;
                if b_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, b_register as u32) })?;
                }
                if a_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, a_register as u32) })?;
                }
                Ok(())
            }
            Value::Cond(condition, true_value, false_value) => {
                let mut allocator = RegisterAllocator::new_with_avoids(&[target_register]);
                let condition_register = allocator.next()?;
                let true_register = allocator.next()?;
                let false_register = allocator.next()?;
                let (condition, condition_saved) =
                    self.emit_intermediate(condition_register, condition)?;
                let (true_value, true_saved) = self.emit_intermediate(true_register, true_value)?;
                let (false_value, false_saved) =
                    self.emit_intermediate(false_register, false_value)?;
                check_asm_result(unsafe {
                    asm_cond(
                        self.asm,
                        target_register as u32,
                        condition,
                        true_value,
                        false_value,
                    )
                })?;
                if false_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, false_register as u32) })?;
                }
                if true_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, true_register as u32) })?;
                }
                if condition_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, condition_register as u32) })?;
                }
                Ok(())
            }
            Value::Load(address, size) => {
                let mut allocator = RegisterAllocator::new_with_avoids(&[target_register]);
                let address_register = allocator.next()?;
                let (address, address_saved) = self.emit_intermediate(address_register, address)?;
                check_asm_result(unsafe {
                    asm_memory_read(self.asm, target_register as u32, address, (*size) as u32)
                })?;
                if address_saved {
                    check_asm_result(unsafe { asm_pop(self.asm, address_register as u32) })?;
                }
                Ok(())
            }
        }
    }

    // If tested value is a register value or immediate value, this function
    // returns converted AsmValue directly. Otherwise it would push the register
    // value in candidate_register to the stack, and then emit the code used
    // to generate value.
    fn emit_intermediate(
        &mut self,
        candidate_register: usize,
        value: &Value,
    ) -> Result<(AsmValue, bool), Error> {
        match value {
            Value::Register(reg) => Ok((register_to_asm_value(*reg), false)),
            Value::Imm(imm) => Ok((immediate_to_asm_value(*imm), false)),
            _ => {
                check_asm_result(unsafe { asm_push(self.asm, candidate_register as u32) })?;
                self.emit(candidate_register, value)?;
                Ok((register_to_asm_value(candidate_register), true))
            }
        }
    }
}
