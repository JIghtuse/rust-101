// Rust-101, Part 01: Expressions, Inherent methods
// ================================================

// ## Expression-based programming
fn sqr(i: i32) -> i32 { i * i }

// Conditionals are also just expressions. This is comparable to the ternary `? :` operator
// from languages like C.
fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

// It is even the case that blocks are expressions, evaluating to the last expression they contain.
fn compute_stuff(x: i32) -> i32 {
    let y = { let z = x*x; z + 14 };
    y*y
}

fn vec_min(v: &Vec<i32>) -> Option<i32> {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }

    let mut min = None;
    for e in v {
        min = Some(match min {
            None => *e,
            Some(n) => min_i32(n, *e)
        })
    }
    min
}

fn vec_sum(v: &Vec<i32>) -> Option<i32> {
    let mut sum = None;
    for item in v {
        sum = match sum {
            None => Some(*item),
            Some(n) => Some(*item + n),
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
    println!("Minimum of a vec: {:?}", min);
    let sum = vec_sum(&vec);
    println!("Sum of a vec: {:?}", sum);
    vec_print(&vec);
}
