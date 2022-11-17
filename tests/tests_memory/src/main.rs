pub mod misc;

use misc::get_current_memory;

fn test() {
    let mut buffer = Vec::<u8>::new();
    println!("--2, mem: {}", get_current_memory());
    buffer.resize(1024 * 1024 * 2, 1);
    println!("--3, mem: {}", get_current_memory());

    for i in 0..buffer.len() {
        if buffer[i] != 1 {
            break;
        }
    }
}

fn tests() {
    for _ in 0..100 {
        test();
    }
}

fn main() {
    println!("--1, mem: {}", get_current_memory());

    tests();
    println!("--4, mem: {}", get_current_memory());
}
