// Rust-101, Part 00: Algebraic datatypes
// ======================================

// As our first piece of Rust code, we want to write a function that computes the
// minimum of a list.


// An `enum` for "a number or nothing" could look as follows:
enum NumberOrNothing {
    Number(i32),
    Nothing
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    use self::NumberOrNothing::{Number,Nothing};
    let mut min = Nothing;

    for item in vec {
        match min {
            Nothing => {
                min = Number(item)
            },
            Number(n) => {
                min = Number(min_i32(item, n))
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

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        NumberOrNothing::Nothing => println!("No minimum value"),
        NumberOrNothing::Number(n) => println!("Minimum of a vec: {}", n),
    }
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}
