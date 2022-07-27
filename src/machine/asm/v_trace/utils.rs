use std::arch::asm;
use std::ptr;

#[inline(never)]
pub fn msbc_256(a: *const u8, b: *const u8) -> bool {
    let mut result: u64;
    unsafe {
        asm!(
            "mov r8, [{1} + 0]",
            "mov r9, [{1} + 8]",
            "mov r10, [{1} + 16]",
            "mov r11, [{1} + 24]",
            "sub r8, [{2} + 0]",
            "sbb r9, [{2} + 8]",
            "sbb r10, [{2} + 16]",
            "sbb r11, [{2} + 24]",
            "sbb {0}, {0}",
            out(reg) result,
            in(reg) a as usize,
            in(reg) b as usize,
        );
    }
    result != 0
}

pub fn narrowing_right_shift_512(src: *const u8, dst: *mut u8, shift: u32, len: usize) {
    let shift = shift & 511;
    let qword_shift = (shift % 64) as usize;
    let start = (shift / 64) as usize;

    for i in 0..len {
        let src = unsafe { (src as *const u64).add(i * 8) };
        let dst = unsafe { (dst as *mut u64).add(i * 4) };

        let mut values = [0u64; 5];

        let len = std::cmp::min(5, 8 - start);
        let read_slice = ptr::slice_from_raw_parts(unsafe { src.add(start) }, len);
        values[0..len].copy_from_slice(unsafe { &*read_slice });

        if qword_shift > 0 {
            let shift1 = values[1] << (64 - qword_shift);
            let shift2 = values[2] << (64 - qword_shift);
            let shift3 = values[3] << (64 - qword_shift);
            let shift4 = values[4] << (64 - qword_shift);

            values[0] = (values[0] >> qword_shift) | shift1;
            values[1] = (values[1] >> qword_shift) | shift2;
            values[2] = (values[2] >> qword_shift) | shift3;
            values[3] = (values[3] >> qword_shift) | shift4;
        }

        let write_slice = ptr::slice_from_raw_parts_mut(dst, 4);
        unsafe { &mut *write_slice }.copy_from_slice(&values[0..4]);
    }
}

#[inline(never)]
pub fn wrapping_add_512(a: *const u8, b: *const u8, dst: *mut u8, len: usize) {
    for i in 0..len {
        unsafe {
            asm!(
                "mov r8, [rsi + 0]",
                "mov r9, [rsi + 8]",
                "mov r10, [rsi + 16]",
                "mov r11, [rsi + 24]",
                "add r8, [rcx + 0]",
                "adc r9, [rcx + 8]",
                "adc r10, [rcx + 16]",
                "adc r11, [rcx + 24]",
                "mov [rdi + 0], r8",
                "mov [rdi + 8], r9",
                "mov [rdi + 16], r10",
                "mov [rdi + 24], r11",
                "mov r8, [rsi + 32]",
                "mov r9, [rsi + 40]",
                "mov r10, [rsi + 48]",
                "mov r11, [rsi + 56]",
                "adc r8, [rcx + 32]",
                "adc r9, [rcx + 40]",
                "adc r10, [rcx + 48]",
                "adc r11, [rcx + 56]",
                "mov [rdi + 32], r8",
                "mov [rdi + 40], r9",
                "mov [rdi + 48], r10",
                "mov [rdi + 56], r11",
                in("rsi") a as usize + i * 64,
                in("rcx") b as usize + i * 64,
                in("rdi") dst as usize + i * 64,
                clobber_abi("sysv64"),
                clobber_abi("win64"),
            );
        }
    }
}

#[inline(never)]
pub fn wrapping_sub_256(a: *const u8, b: *const u8, dst: *mut u8, len: usize) {
    for i in 0..len {
        unsafe {
            asm!(
                "mov r8, [rsi + 0]",
                "mov r9, [rsi + 8]",
                "mov r10, [rsi + 16]",
                "mov r11, [rsi + 24]",
                "sub r8, [rcx + 0]",
                "sbb r9, [rcx + 8]",
                "sbb r10, [rcx + 16]",
                "sbb r11, [rcx + 24]",
                "mov [rdi + 0], r8",
                "mov [rdi + 8], r9",
                "mov [rdi + 16], r10",
                "mov [rdi + 24], r11",
                in("rsi") a as usize + i * 32,
                in("rcx") b as usize + i * 32,
                in("rdi") dst as usize + i * 32,
                clobber_abi("sysv64"),
                clobber_abi("win64"),
            );
        }
    }
}

#[inline(never)]
pub fn wrapping_add_256(a: *const u8, b: *const u8, dst: *mut u8, len: usize) {
    for i in 0..len {
        unsafe {
            asm!(
                "mov r8, [rsi + 0]",
                "mov r9, [rsi + 8]",
                "mov r10, [rsi + 16]",
                "mov r11, [rsi + 24]",
                "add r8, [rcx + 0]",
                "adc r9, [rcx + 8]",
                "adc r10, [rcx + 16]",
                "adc r11, [rcx + 24]",
                "mov [rdi + 0], r8",
                "mov [rdi + 8], r9",
                "mov [rdi + 16], r10",
                "mov [rdi + 24], r11",
                in("rsi") a as usize + i * 32,
                in("rcx") b as usize + i * 32,
                in("rdi") dst as usize + i * 32,
                clobber_abi("sysv64"),
                clobber_abi("win64"),
            );
        }
    }
}

#[inline(never)]
pub fn widening_mul_256_non_overlapping(a: *const u8, b: *const u8, dst: *mut u8, len: usize) {
    debug_assert!(a != b);
    debug_assert!(a != dst);
    debug_assert!(b != dst);

    for i in 0..len {
        // Inspired from https://github.com/cloudflare/bn256/blob/9bd9f73a0273ed2f42707ed13b3e36d38baa2a49/mul_amd64.h#L1
        unsafe {
            asm!(
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 0]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "mov [rdi + 0], r8",
                "mov [rdi + 8], r9",
                "mov [rdi + 16], r10",
                "mov [rdi + 24], r11",
                "mov [rdi + 32], r12",
                "",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 8]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "add r8, [rdi + 8]",
                "adc r9, [rdi + 16]",
                "adc r10, [rdi + 24]",
                "adc r11, [rdi + 32]",
                "adc r12, 0",
                "mov [rdi + 8], r8",
                "mov [rdi + 16], r9",
                "mov [rdi + 24], r10",
                "mov [rdi + 32], r11",
                "mov [rdi + 40], r12",
                "",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 16]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "add r8, [rdi + 16]",
                "adc r9, [rdi + 24]",
                "adc r10, [rdi + 32]",
                "adc r11, [rdi + 40]",
                "adc r12, 0",
                "mov [rdi + 16], r8",
                "mov [rdi + 24], r9",
                "mov [rdi + 32], r10",
                "mov [rdi + 40], r11",
                "mov [rdi + 48], r12",
                "",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 0]",
                "mov r8, rax",
                "mov r9, rdx",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 8]",
                "add r9, rax",
                "adc rdx, 0",
                "mov r10, rdx",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 16]",
                "add r10, rax",
                "adc rdx, 0",
                "mov r11, rdx",
                "mov rax, [rsi + 24]",
                "mul qword ptr [rcx + 24]",
                "add r11, rax",
                "adc rdx, 0",
                "mov r12, rdx",
                "",
                "add r8, [rdi + 24]",
                "adc r9, [rdi + 32]",
                "adc r10, [rdi + 40]",
                "adc r11, [rdi + 48]",
                "adc r12, 0",
                "mov [rdi + 24], r8",
                "mov [rdi + 32], r9",
                "mov [rdi + 40], r10",
                "mov [rdi + 48], r11",
                "mov [rdi + 56], r12",
                in("rsi") a as usize + i * 32,
                in("rcx") b as usize + i * 32,
                in("rdi") dst as usize + i * 64,
                lateout("r12") _,
                clobber_abi("sysv64"),
                clobber_abi("win64"),
            );
        }
    }
}
