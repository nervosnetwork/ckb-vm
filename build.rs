// Due to this bug: https://github.com/rust-lang/cargo/issues/4866, we cannot
// specify different features based on different targets now in cargo file. We
// have to keep features always on, and do conditional compilation within the
// source code

use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let support_asm = ["aarch64", "riscv64", "x86_64"].contains(&target_arch.as_str());
    if !support_asm {
        if cfg!(feature = "asm") {
            panic!("Asm is not available for {}!", target_arch);
        }
        return;
    }
    let request_asm = cfg!(feature = "asm") || cfg!(feature = "detect-asm");
    if !request_asm {
        return;
    }

    println!("cargo:rustc-cfg=has_asm");
    println!("cargo:rerun-if-changed=src/machine/asm/execute_x64.S");
    println!("cargo:rerun-if-changed=src/machine/asm/execute_aarch64.S");
    println!("cargo:rerun-if-changed=src/machine/asm/execute_riscv64.S");
    println!("cargo:rerun-if-changed=src/machine/asm/cdefinitions_generated.h");

    match target_arch.as_str() {
        "aarch64" => {
            cc::Build::new()
                .include("src/machine/asm")
                .file("src/machine/asm/execute_aarch64.S")
                .compile("asm");
        }
        "riscv64" => {
            cc::Build::new()
                .include("src/machine/asm")
                .file("src/machine/asm/execute_riscv64.S")
                .compile("asm");
        }
        "x86_64" => {
            cc::Build::new()
                .include("src/machine/asm")
                .file("src/machine/asm/execute_x64.S")
                .compile("asm");
        }
        _ => {}
    }
}
