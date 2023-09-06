#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
use std::io;

fn main() {
    let mut buffer = Buffer { data: vec![1, 2, 3] };
    let sum = buffer.sum();
    println!("Sum: {:?}", sum);
    let mut buffer = Buffer { data: vec![1.0, 2.0, 3.0] };
    let sum = buffer.sum();
    println!("Sum: {:?}", sum);
}