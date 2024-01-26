// Due to this bug: https://github.com/rust-lang/cargo/issues/4866, we cannot
// specify different features based on different targets now in cargo file. We
// have to keep features always on, and do conditional compilation within the
// source code

fn main() {
    use std::env;

    let version = env::var("CARGO_PKG_VERSION")
        .unwrap()
        .replace(['.', '-'], "_");
    println!("cargo:rustc-env=CKB_VM_VERSION={version}");

    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let is_windows = target_family == "windows";
    let is_msvc = is_windows && (target_env == "msvc");
    let is_unix = target_family == "unix";
    let is_x86_64 = target_arch == "x86_64";
    let is_aarch64 = target_arch == "aarch64";
    let x64_asm = is_x86_64 && (is_windows || is_unix);
    let aarch64_asm = is_aarch64 && is_unix;
    let can_enable_asm = x64_asm || aarch64_asm;

    if cfg!(feature = "asm") && (!can_enable_asm) {
        panic!(
            "Asm feature is not available for target {} on {}!",
            target_arch, target_family
        );
    }

    if cfg!(any(feature = "asm", feature = "detect-asm")) && can_enable_asm {
        println!("cargo:rerun-if-changed=src/machine/asm/execute_x64.S");
        println!("cargo:rerun-if-changed=src/machine/asm/execute_aarch64.S");
        println!("cargo:rerun-if-changed=src/machine/asm/cdefinitions_generated.h");

        let mut build = cc::Build::new();

        if x64_asm {
            build
                .define("CKB_VM_VERSION", &*version)
                .file("src/machine/asm/execute_x64.S");
            if is_msvc {
                // For now, only an assembly source code is required for CKB-VM, we won't
                // need to build any C source file here. Hence we can use this simpler solution
                // to set the default compiler to GCC. We will need to manually trigger the
                // command to assemble the assembly code file, should any C source file is also
                // required here.
                build.compiler("gcc");
            }
        } else if aarch64_asm {
            build
                .define("CKB_VM_VERSION", &*version)
                .file("src/machine/asm/execute_aarch64.S");
        }

        build.include("src/machine/asm").compile("asm");

        println!("cargo:rustc-cfg=has_asm")
    }
}
