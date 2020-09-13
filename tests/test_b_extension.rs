mod machine_run;

#[test]
pub fn test_b_extension() {
    machine_run::int_v1_imcb("tests/programs/b_extension");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/b_extension");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/b_extension");
}

#[test]
pub fn test_clzw_bug() {
    machine_run::int_v1_imcb("tests/programs/clzw_bug");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/clzw_bug");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/clzw_bug");
}

#[test]
pub fn test_packw_signextend() {
    machine_run::int_v1_imcb("tests/programs/packw_signextend");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/packw_signextend");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/packw_signextend");
}

#[test]
pub fn test_single_bit_signextend() {
    machine_run::int_v1_imcb("tests/programs/single_bit_signextend");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/single_bit_signextend");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/single_bit_signextend");
}

#[test]
pub fn test_sbinvi_aot_load_imm_bug() {
    machine_run::int_v1_imcb("tests/programs/sbinvi_aot_load_imm_bug");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/sbinvi_aot_load_imm_bug");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/sbinvi_aot_load_imm_bug");
}

#[test]
pub fn test_rorw_in_end_of_aot_block() {
    // The 1024th instruction will use one more temporary register than normal.
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/rorw_in_end_of_aot_block");
}

#[test]
pub fn test_fsri_decode_bug() {
    machine_run::int_v1_imcb("tests/programs/fsri_decode_bug");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/fsri_decode_bug");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/fsri_decode_bug");
}
