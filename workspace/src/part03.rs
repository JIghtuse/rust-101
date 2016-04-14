// Rust-101, Part 03: Input
// ========================


// I/O is provided by the module `std::io`, so we first have import that with `use`.
// We also import the I/O *prelude*, which makes a bunch of commonly used I/O stuff
// directly available.
use std::io::prelude::*;
use std::io;
use std::fmt::Display;

pub trait Print {
    fn print(self);
}

impl<T: Print> SomethingOrNothing<T> {
    fn print2(self) {
        match self {
            Nothing => print!("<no value>"),
            Something(n) => n.print(),
        }
    }
}

impl Print for i32 {
    fn print(self) {
        print!("{}", self);
    }
}

impl Print for f32 {
    fn print(self) {
        print!("{}", self);
    }
}

fn read_vec() -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    // The central handle to the standard input is made available by the function `io::stdin`.
    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<i32>() {
            Ok(num) => {
                vec.push(num);
            },
            Err(_) => {
                println!("What did I say about numbers?");
            },
        }
    }

    vec
}

fn read_vec_float() -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::new();
    // The central handle to the standard input is made available by the function `io::stdin`.
    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<f32>() {
            Ok(num) => {
                vec.push(num);
            },
            Err(_) => {
                println!("What did I say about numbers?");
            },
        }
    }
    vec
}


use part02::{SomethingOrNothing,Something,Nothing,vec_min};

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print2();
    let nothing : SomethingOrNothing<i32> =Nothing;
    nothing.print2();

    let v_float = read_vec_float();
    let min_float = vec_min(v_float);
    min_float.print2();
}
