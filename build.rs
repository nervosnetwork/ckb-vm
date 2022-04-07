// Due to this bug: https://github.com/rust-lang/cargo/issues/4866, we cannot
// specify different features based on different targets now in cargo file. We
// have to keep features always on, and do conditional compilation within the
// source code

fn main() {
    use std::env;

    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let is_windows = target_family == "windows";
    let is_unix = target_family == "unix";
    let is_x86_64 = target_arch == "x86_64";
    let is_aarch64 = target_arch == "aarch64";
    let x64_asm = is_x86_64 && (is_windows || is_unix);
    let aarch64_asm = is_aarch64 && is_unix;
    let can_enable_asm = x64_asm || aarch64_asm;
    let can_enable_aot = x64_asm;

    if cfg!(feature = "asm") && (!can_enable_asm) {
        panic!(
            "Asm feature is not available for target {} on {}!",
            target_arch, target_family
        );
    }

    if cfg!(feature = "aot") && (!can_enable_aot) {
        panic!(
            "Aot feature is not available for target {} on {}!",
            target_arch, target_family
        );
    }

    if cfg!(any(feature = "asm", feature = "detect-asm")) && can_enable_asm {
        use cc::Build;
        use std::path::Path;
        use std::process::Command;

        let enable_aot = cfg!(feature = "aot");

        fn run_command(mut c: Command) {
            println!("Running Command[{:?}]", c);

            let output = c.output().unwrap_or_else(|e| {
                panic!("Error running Command[{:?}], error: {:?}", c, e);
            });

            if !output.status.success() {
                use std::io::{self, Write};
                io::stdout()
                    .write_all(&output.stdout)
                    .expect("stdout write");
                io::stderr()
                    .write_all(&output.stderr)
                    .expect("stderr write");

                panic!(
                    "Command[{:?}] exits with non-success status: {:?}",
                    c, output.status
                );
            }
        }

        let mut build = Build::new();

        if is_windows && x64_asm {
            let out_dir = env::var("OUT_DIR").unwrap();
            let expand_path = Path::new(&out_dir).join("execute_x64-expanded.S");
            let mut expand_command = Command::new("clang");
            expand_command
                .arg("-E")
                .arg("src/machine/asm/execute_x64.S")
                .arg("-o")
                .arg(&expand_path);
            run_command(expand_command);

            let compile_path = Path::new(&out_dir).join("execute_x64.o");
            let mut compile_command = Command::new("yasm");
            compile_command.env_clear();
            compile_command
                .arg("-p")
                .arg("gas")
                .arg("-f")
                .arg("x64")
                .arg("-m")
                .arg("amd64")
                .arg(&expand_path)
                .arg("-o")
                .arg(&compile_path);
            run_command(compile_command);

            build.object(&compile_path);

            if enable_aot {
                build.file("src/machine/aot/aot.x64.win.compiled.c");
            }
        } else if x64_asm {
            build.file("src/machine/asm/execute_x64.S");

            if enable_aot {
                build.file("src/machine/aot/aot.x64.compiled.c");
            }
        } else if aarch64_asm {
            build.file("src/machine/asm/execute_aarch64.S");
            // TODO: AOT
        }

        if enable_aot {
            build.include("dynasm");
            println!("cargo:rustc-cfg=has_aot");
        }

        build.include("src/machine/asm").compile("asm");

        println!("cargo:rustc-cfg=has_asm")
    }
}
