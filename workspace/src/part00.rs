// Rust-101, Part 00: Algebraic datatypes
// ======================================

// As our first piece of Rust code, we want to write a function that computes the
// minimum of a list.

fn vec_min(vec: Vec<i32>) -> Option<i32> {
    let mut min = None;

    for item in vec {
        min = 
        match min {
            None => {
                Some(item)
            },
            Some(n) => {
                Some(min_i32(item, n))
            }
        }
    }
    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// TODO: implement reading from the stdin/file
fn read_vec() -> Vec<i32> {
    vec![5, 2, 1, 0, 3, 4]
}

fn print_number_or_nothing(n: Option<i32>) {
    match n {
        None => println!("No minimum value"),
        Some(n) => println!("Minimum of a vec: {}", n),
    }
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}
