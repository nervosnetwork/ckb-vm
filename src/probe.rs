use crate::instructions::Register;
use crate::machine::Machine;
use crate::registers::{A0, A1, A2, A3, A4, A5};

pub fn probe_jump<Mac: Machine>(machine: &mut Mac, link: Mac::REG, next_pc: Mac::REG) {
    let regs = machine.registers_ptr();
    let memory = machine.memory_ptr();
    probe::probe!(ckb_vm, jump, link.to_u64(), next_pc.to_u64(), regs, memory);
}

pub fn probe_syscall<Mac: Machine>(machine: &mut Mac, code: u64) {
    let arg0 = machine.registers()[A0].to_u64();
    let arg1 = machine.registers()[A1].to_u64();
    let arg2 = machine.registers()[A2].to_u64();
    let arg3 = machine.registers()[A3].to_u64();
    let arg4 = machine.registers()[A4].to_u64();
    let arg5 = machine.registers()[A5].to_u64();
    probe::probe!(ckb_vm, syscall, code, arg0, arg1, arg2, arg3, arg4, arg5);
}

pub fn probe_syscall_return<Mac: Machine>(machine: &mut Mac, code: u64) {
    let ret_code = machine.registers()[A0].to_u64();
    let ret_code2 = machine.registers()[A1].to_u64();
    probe::probe!(ckb_vm, syscall_ret, code, ret_code, ret_code2);
}
