// Rust-101, Part 01: Expressions, Inherent methods
// ================================================

// ## Expression-based programming
fn sqr(i: i32) -> i32 { i * i }

// Conditionals are also just expressions. This is comparable to the ternary `? :` operator
// from languages like C.
fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

enum NumberOrNothing {
    Number(i32),
    Nothing
}
use self::NumberOrNothing::{Number,Nothing};

// ## Inherent implementations
impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
    fn number_or_default(self, default: i32) -> i32 {
        match self {
            Nothing => default,
            Number(n) => n,
        }
    }
}

// It is even the case that blocks are expressions, evaluating to the last expression they contain.
fn compute_stuff(x: i32) -> i32 {
    let y = { let z = x*x; z + 14 };
    y*y
}

fn vec_min(v: &Vec<i32>) -> NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }

    let mut min = Nothing;
    for e in v {
        min = Number(match min {
            Nothing => *e,
            Number(n) => min_i32(n, *e)
        })
    }
    min
}

fn vec_sum(v: &Vec<i32>) -> NumberOrNothing {
    let mut sum = Nothing;
    for item in v {
        sum = match sum {
            Nothing => Number(*item),
            Number(n) => Number(*item + n),
        }
    }
    sum
}

fn vec_print(v: &Vec<i32>) {
    for item in v {
        print!("{} ", item);
    }
    println!("");
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,2,9,27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(&vec);
    min.print();
    let sum = vec_sum(&vec);
    sum.print();
    vec_print(&vec);
}
