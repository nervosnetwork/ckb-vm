// Due to this bug: https://github.com/rust-lang/cargo/issues/4866, we cannot
// specify different features based on different targets now in cargo file. We
// have to keep features always on, and do conditional compilation within the
// source code
#[cfg(all(unix, target_pointer_width = "64", feature = "asm"))]
use cc::Build;

#[cfg(all(unix, target_pointer_width = "64", feature = "asm"))]
fn main() {
    let mut build = Build::new();

    build
        .file("src/machine/asm/execute.S")
        .file("src/machine/aot/aot.x64.compiled.c")
        .include("dynasm")
        .include("src/machine/asm")
        .compile("asm");
}

#[cfg(not(all(unix, target_pointer_width = "64", feature = "asm")))]
fn main() {}
