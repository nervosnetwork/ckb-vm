// Due to this bug: https://github.com/rust-lang/cargo/issues/4866, we cannot
// specify different features based on different targets now in cargo file. We
// have to keep features always on, and do conditional compilation within the
// source code
#[cfg(all(unix, target_pointer_width = "64", any(feature = "asm", feature = "jit")))]
use cc::Build;

fn main() {
    #[cfg(all(unix, target_pointer_width = "64", any(feature = "asm", feature = "jit")))]
    let mut build = Build::new();

    #[cfg(all(unix, target_pointer_width = "64", feature = "asm"))]
    build.file("src/machine/asm/execute.S");

    #[cfg(all(unix, target_pointer_width = "64", feature = "jit"))]
    build.file("src/jit/asm.x64.compiled.c").include("dynasm");

    #[cfg(all(unix, target_pointer_width = "64", any(feature = "asm", feature = "jit")))]
    build.compile("asm");
}
