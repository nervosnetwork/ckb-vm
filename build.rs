#[cfg(feature = "jit")]
use cc::Build;

fn main() {
    #[cfg(all(unix, target_pointer_width = "64", feature = "jit"))]
    Build::new()
        .file("src/jit/asm.x64.compiled.c")
        .include("dynasm")
        .compile("asm");
}
