#![no_main]
risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;

use std::hint::black_box;

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}

fn main() {
    // read the input
    let input: u32 = env::read();

    let result = black_box(fibonacci(black_box(input)));
    println!("result: {}", result);
}
