// Example "is-thirteen" is a useless(almost) program for just checking if a number is equal to 13.
//
// But it's also important because it shows how to compile a program from C source code and then run
// it by ckb-vm. So, Let's start.
//
// To build the "is13.c" to riscv output, you will need to install "riscv-gnu-toolchain" in your system.
//
//   $ git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
//   $ cd riscv-gnu-toolchain
//   $ mkdir build && cd build
//   $ ../configure --prefix=/opt/riscv --with-arch=rv32imac --with-abi=ilp32
//   $ make
//
// On my ubuntu machine, it takes 1 hours. Sad.
//
// Then, you can build "is13.c" by "riscv32-unknown-elf-gcc"
//
//   $ riscv32-unknown-elf-gcc -o is13 is13.c
//
// Where can you find the "riscv32-unknown-elf-gcc", depending on the previous "../configure --prefix=xxxx"
//
// Now, you have the "is13" binary! Copy the file to this directory, call it by ckb-vm, as shown in the Rust
// code below. And feel free to run this example by command "cargo":
//
//     $ cargo run --example is13 13
//  or $ cargo run --example is13 0xd
//  or $ cargo run --example is13 HELLO
use bytes::Bytes;
use std::io::Read;

fn main() {
    let args: Vec<Bytes> = std::env::args().map(|a| a.into()).collect();

    let mut file = std::fs::File::open("examples/is13").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = Bytes::from(buffer);

    let r = ckb_vm::run::<u32, ckb_vm::SparseMemory<u32>>(&buffer, &args[..]).unwrap();
    match r {
        1 => println!("{:?} is not thirteen", args[1]),
        0 => println!("{:?} is thirteen", args[1]),
        _ => panic!(""),
    }
}
