#[cfg(any(feature = "asm", feature = "jit"))]
use cc::Build;

fn main() {
    #[cfg(any(feature = "asm", feature = "jit"))]
    let mut build = Build::new();

    #[cfg(all(unix, target_pointer_width = "64", feature = "asm"))]
    build.file("src/machine/asm/execute.S");

    #[cfg(all(unix, target_pointer_width = "64", feature = "jit"))]
    build.file("src/jit/asm.x64.compiled.c").include("dynasm");

    #[cfg(any(feature = "asm", feature = "jit"))]
    build.compile("asm");
}
