use super::{
    super::super::instructions::ast::{ActionOp1, ActionOp2, SignActionOp2, Value},
    Write,
};
use crate::Error;
use libc::{c_int, c_void, size_t};
use std::collections::HashMap;

// This is a C struct type
#[repr(C)]
struct AotContext {
    _private: [u8; 0],
}

#[repr(C)]
union AotValueContent {
    reg: u32,
    i: u64,
    // This is an internal field only used in C side
    _x64_reg: u32,
}

const AOT_TAG_REGISTER: u32 = 0x1;
const AOT_TAG_IMMEDIATE: u32 = 0x2;

const MINIMAL_TEMP_REGISTER: usize = 32;
const MAXIMAL_TEMP_REGISTER: usize = 34;

#[repr(C)]
struct AotValue {
    tag: u32,
    value: AotValueContent,
}

extern "C" {
    fn aot_new(npc: u32, version: u32) -> *mut AotContext;
    fn aot_finalize(c: *mut AotContext);
    fn aot_link(c: *mut AotContext, szp: *mut size_t) -> c_int;
    fn aot_encode(c: *mut AotContext, buffer: *mut c_void) -> c_int;

    fn aot_getpclabel(c: *mut AotContext, label: u32, offset: *mut u32) -> c_int;
    fn aot_label(c: *mut AotContext, label: u32) -> c_int;
    fn aot_add_cycles(c: *mut AotContext, cycles: u64) -> c_int;
    fn aot_ecall(c: *mut AotContext) -> c_int;
    fn aot_ebreak(c: *mut AotContext) -> c_int;
    fn aot_mov(c: *mut AotContext, target: u32, value: AotValue) -> c_int;
    fn aot_add(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_sub(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_mul(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_mulh(
        c: *mut AotContext,
        target: u32,
        a: AotValue,
        b: AotValue,
        is_signed: c_int,
    ) -> c_int;
    fn aot_mulhsu(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_div(
        c: *mut AotContext,
        target: u32,
        a: AotValue,
        b: AotValue,
        is_signed: c_int,
    ) -> c_int;
    fn aot_rem(
        c: *mut AotContext,
        target: u32,
        a: AotValue,
        b: AotValue,
        is_signed: c_int,
    ) -> c_int;
    fn aot_and(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_or(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_not(c: *mut AotContext, target: u32, a: AotValue, logical: c_int) -> c_int;
    fn aot_xor(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_shl(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_shr(
        c: *mut AotContext,
        target: u32,
        a: AotValue,
        b: AotValue,
        is_signed: c_int,
    ) -> c_int;
    fn aot_eq(c: *mut AotContext, target: u32, a: AotValue, b: AotValue) -> c_int;
    fn aot_lt(c: *mut AotContext, target: u32, a: AotValue, b: AotValue, is_signed: c_int)
        -> c_int;
    fn aot_cond(
        c: *mut AotContext,
        target: u32,
        condition: AotValue,
        true_value: AotValue,
        false_value: AotValue,
    ) -> c_int;
    fn aot_extend(
        c: *mut AotContext,
        target: u32,
        a: AotValue,
        b: AotValue,
        is_signed: c_int,
    ) -> c_int;

    fn aot_mov_pc(c: *mut AotContext, value: AotValue) -> c_int;
    fn aot_cond_pc(
        c: *mut AotContext,
        condition: AotValue,
        true_value: AotValue,
        false_value: AotValue,
    ) -> c_int;

    fn aot_memory_read(c: *mut AotContext, target: u32, addr: AotValue, size: u32) -> c_int;
    fn aot_memory_write(c: *mut AotContext, addr: AotValue, v: AotValue, size: u32) -> c_int;
}

fn immediate_to_aot_value(imm: u64) -> AotValue {
    AotValue {
        tag: AOT_TAG_IMMEDIATE,
        value: AotValueContent { i: imm },
    }
}

fn register_to_aot_value(register: usize) -> AotValue {
    AotValue {
        tag: AOT_TAG_REGISTER,
        value: AotValueContent {
            reg: register as u32,
        },
    }
}

fn check_aot_result(result: c_int) -> Result<(), Error> {
    if result == 0 {
        Ok(())
    } else {
        Err(Error::Dynasm(result))
    }
}

struct TempRegisterAllocator {
    next: usize,
}

impl Default for TempRegisterAllocator {
    fn default() -> TempRegisterAllocator {
        TempRegisterAllocator {
            next: MINIMAL_TEMP_REGISTER,
        }
    }
}

impl TempRegisterAllocator {
    pub fn next(&mut self) -> Result<usize, Error> {
        if self.next > MAXIMAL_TEMP_REGISTER {
            return Err(Error::OutOfBound);
        }
        let value = self.next;
        self.next += 1;
        Ok(value)
    }

    pub fn save(&mut self) -> usize {
        self.next
    }

    pub fn restore(&mut self, v: usize) {
        self.next = v;
    }

    pub fn clear(&mut self) {
        self.next = MINIMAL_TEMP_REGISTER;
    }
}

pub struct Emitter {
    aot: *mut AotContext,
    allocator: TempRegisterAllocator,
}

impl Drop for Emitter {
    fn drop(&mut self) {
        unsafe {
            aot_finalize(self.aot);
        }
    }
}

impl Emitter {
    pub fn new(labels: usize, version: u32) -> Result<Emitter, Error> {
        let aot = unsafe { aot_new(labels as u32, version) };
        if aot.is_null() {
            Err(Error::Dynasm(-1))
        } else {
            let emitter = Emitter {
                aot,
                allocator: TempRegisterAllocator::default(),
            };
            Ok(emitter)
        }
    }

    pub fn link(&mut self) -> Result<usize, Error> {
        let mut buffer_size: usize = 0;
        let result = unsafe { aot_link(self.aot, &mut buffer_size) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(buffer_size)
    }

    pub fn encode(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let result = unsafe { aot_encode(self.aot, buffer.as_mut_ptr() as *mut c_void) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(())
    }

    pub fn get_label_offset(&mut self, label: u32) -> Result<u32, Error> {
        let mut offset = 0;
        let result = unsafe { aot_getpclabel(self.aot, label, &mut offset as *mut u32) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(offset)
    }

    pub fn emit_label(&mut self, label: u32) -> Result<(), Error> {
        let result = unsafe { aot_label(self.aot, label) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(())
    }

    pub fn emit_add_cycles(&mut self, cycles: u64) -> Result<(), Error> {
        let result = unsafe { aot_add_cycles(self.aot, cycles) };
        if result != 0 {
            return Err(Error::Dynasm(result));
        }
        Ok(())
    }

    // Emit a series of writes atomically
    pub fn emit_writes(&mut self, writes: &[Write]) -> Result<(), Error> {
        let saved = self.allocator.save();
        let mut register_writes: HashMap<usize, usize> = HashMap::default();
        let mut pc_write: Option<AotValue> = None;
        for write in writes {
            match write {
                Write::Register { index, value } => {
                    let register = self.allocator.next()?;
                    self.emit_register_write(register, value)?;
                    register_writes.insert(*index, register);
                }
                Write::Pc { value } => {
                    pc_write = Some(self.emit_value(value)?);
                }
                _ => {
                    return Err(Error::Unexpected);
                }
            }
        }
        for (target, source) in register_writes {
            check_aot_result(unsafe {
                aot_mov(self.aot, target as u32, register_to_aot_value(source))
            })?;
        }
        if let Some(pc_value) = pc_write {
            check_aot_result(unsafe { aot_mov_pc(self.aot, pc_value) })?;
        }
        self.allocator.restore(saved);
        Ok(())
    }

    pub fn emit(&mut self, write: &Write) -> Result<(), Error> {
        match write {
            Write::Memory {
                address,
                size,
                value,
            } => self.emit_memory_write(address, *size, value),
            Write::Register { index, value } => self.emit_register_write(*index, value),
            Write::Pc { value } => self.emit_pc_write(value),
            Write::Ecall => check_aot_result(unsafe { aot_ecall(self.aot) }),
            Write::Ebreak => check_aot_result(unsafe { aot_ebreak(self.aot) }),
        }?;
        self.allocator.clear();
        Ok(())
    }

    fn emit_memory_write(&mut self, address: &Value, size: u8, value: &Value) -> Result<(), Error> {
        let saved = self.allocator.save();
        let address_value = self.emit_value(address)?;
        let value_value = self.emit_value(value)?;
        check_aot_result(unsafe {
            aot_memory_write(self.aot, address_value, value_value, u32::from(size))
        })?;
        self.allocator.restore(saved);
        Ok(())
    }

    fn emit_pc_write(&mut self, value: &Value) -> Result<(), Error> {
        let saved = self.allocator.save();
        match value {
            // emit_value below will handle the case when we are dealing with
            // a register or an immediate.
            Value::Cond(condition, true_value, false_value) => {
                let condition_value = self.emit_value(condition)?;
                let true_value = self.emit_value(true_value)?;
                let false_value = self.emit_value(false_value)?;
                check_aot_result(unsafe {
                    aot_cond_pc(self.aot, condition_value, true_value, false_value)
                })
            }
            _ => {
                let v = self.emit_value(value)?;
                check_aot_result(unsafe { aot_mov_pc(self.aot, v) })
            }
        }?;
        self.allocator.restore(saved);
        Ok(())
    }

    fn emit_register_write(&mut self, target_register: usize, value: &Value) -> Result<(), Error> {
        match value {
            Value::Register(reg) => {
                if *reg != target_register {
                    check_aot_result(unsafe {
                        aot_mov(
                            self.aot,
                            target_register as u32,
                            register_to_aot_value(*reg),
                        )
                    })
                } else {
                    Ok(())
                }
            }
            Value::Imm(imm) => check_aot_result(unsafe {
                aot_mov(
                    self.aot,
                    target_register as u32,
                    immediate_to_aot_value(*imm),
                )
            }),
            Value::Op1(op, a) => {
                let saved = self.allocator.save();
                let a_value = self.emit_value(a)?;
                let result = match op {
                    ActionOp1::Not => unsafe {
                        aot_not(self.aot, target_register as u32, a_value, 0)
                    },
                    ActionOp1::LogicalNot => unsafe {
                        aot_not(self.aot, target_register as u32, a_value, 1)
                    },
                };
                check_aot_result(result)?;
                self.allocator.restore(saved);
                Ok(())
            }
            Value::Op2(op, a, b) => {
                let saved = self.allocator.save();
                let a_value = self.emit_value(a)?;
                let b_value = self.emit_value(b)?;
                let result = match op {
                    ActionOp2::Add => unsafe {
                        aot_add(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Sub => unsafe {
                        aot_sub(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Mul => unsafe {
                        aot_mul(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Mulhsu => unsafe {
                        aot_mulhsu(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Bitand => unsafe {
                        aot_and(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Bitor => unsafe {
                        aot_or(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Bitxor => unsafe {
                        aot_xor(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Shl => unsafe {
                        aot_shl(self.aot, target_register as u32, a_value, b_value)
                    },
                    ActionOp2::Eq => unsafe {
                        aot_eq(self.aot, target_register as u32, a_value, b_value)
                    },
                };
                check_aot_result(result)?;
                self.allocator.restore(saved);
                Ok(())
            }
            Value::SignOp2(op, a, b, signed) => {
                let saved = self.allocator.save();
                let a_value = self.emit_value(a)?;
                let b_value = self.emit_value(b)?;
                let signed: c_int = (*signed).into();
                let result = match op {
                    SignActionOp2::Mulh => unsafe {
                        aot_mulh(self.aot, target_register as u32, a_value, b_value, signed)
                    },
                    SignActionOp2::Div => unsafe {
                        aot_div(self.aot, target_register as u32, a_value, b_value, signed)
                    },
                    SignActionOp2::Rem => unsafe {
                        aot_rem(self.aot, target_register as u32, a_value, b_value, signed)
                    },
                    SignActionOp2::Shr => unsafe {
                        aot_shr(self.aot, target_register as u32, a_value, b_value, signed)
                    },
                    SignActionOp2::Lt => unsafe {
                        aot_lt(self.aot, target_register as u32, a_value, b_value, signed)
                    },
                    SignActionOp2::Extend => unsafe {
                        aot_extend(self.aot, target_register as u32, a_value, b_value, signed)
                    },
                };
                check_aot_result(result)?;
                self.allocator.restore(saved);
                Ok(())
            }
            Value::Cond(condition, true_value, false_value) => {
                let saved = self.allocator.save();
                let condition_value = self.emit_value(condition)?;
                let true_value = self.emit_value(true_value)?;
                let false_value = self.emit_value(false_value)?;
                check_aot_result(unsafe {
                    aot_cond(
                        self.aot,
                        target_register as u32,
                        condition_value,
                        true_value,
                        false_value,
                    )
                })?;
                self.allocator.restore(saved);
                Ok(())
            }
            Value::Load(address, size) => {
                let saved = self.allocator.save();
                let address_value = self.emit_value(address)?;
                check_aot_result(unsafe {
                    aot_memory_read(
                        self.aot,
                        target_register as u32,
                        address_value,
                        u32::from(*size),
                    )
                })?;
                self.allocator.restore(saved);
                Ok(())
            }
        }
    }

    // Emit value either directly as a direct intermediate, or emit the value
    // to a temporary register. Either way, this method should return an AotValue
    // field that can be used in aot options.
    fn emit_value(&mut self, value: &Value) -> Result<AotValue, Error> {
        match value {
            Value::Register(reg) => Ok(register_to_aot_value(*reg)),
            Value::Imm(imm) => Ok(immediate_to_aot_value(*imm)),
            _ => {
                let register = self.allocator.next()?;
                self.emit_register_write(register, value)?;
                Ok(register_to_aot_value(register))
            }
        }
    }
}
