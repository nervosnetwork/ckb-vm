use cc::Build;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    // Right now, JIT only supports linux and Mac OS on x86_64 CPUs
    if target.contains("x86_64") && (target.contains("linux") || target.contains("apple")) {
        Build::new()
            .file("src/jit/asm.x64.compiled.c")
            .include("dynasm")
            .compile("asm");
    }
}
