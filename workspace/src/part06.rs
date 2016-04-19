// Rust-101, Part 06: Copy, Lifetimes
// ==================================

// We continue to work on our `BigInt`, so we start by importing what we already established.
use part05::BigInt;

// With `BigInt` being about numbers, we should be able to write a version of `vec_min`
// that computes the minimum of a list of `BigInt`. First, we have to write `min` for `BigInt`.
impl BigInt {
    fn min_try1(self, other: Self) -> Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        // Now our assumption of having no trailing zeros comes in handy:
        // If the lengths of the two numbers differ, we already know which is larger.
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            for i in self.data.len() - 1 ... 0 {
                if self.data[i] < other.data[i] {
                    return self;
                } else if self.data[i] > other.data[i] {
                    return other;
                }
            }
            self
        }
    }
}

// Now we can write `vec_min`.
fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
    let mut min: Option<BigInt> = None;
    // If `v` is a shared reference to a vector, then the default for iterating over it is to call `iter`, the iterator that borrows the elements.
    for e in v {
        let e = e.clone();
        min = Some(match min {
            None => e,
            Some(n) => n.min_try1(e)
        });
    }
    min
}

// ## `Copy` types

use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Copy> Copy for SomethingOrNothing<T> {}


// ## Lifetimes

fn head<T>(v: &Vec<T>) -> Option<&T> {
    if v.len() > 0 {
        Some(&v[0])
    } else {
        None
    }
}
// Technically, we are returning a pointer to the first element. But doesn't that mean that callers have to be
// careful? Imagine `head` would be a C++ function, and we would write the following code.
/*
  int foo(std::vector<int> v) {
    int *first = head(v);
    v.push_back(42);
    return *first;
  }
*/
fn rust_foo(mut v: Vec<i32>) -> i32 {
    let first: Option<&i32> = head(&v);
    /* v.push(42); */
    *first.unwrap()
}

pub fn main() {
    let a = BigInt::from_vec(vec![1]);
    let b = BigInt::from_vec(vec![3]);
    println!("min of {:?} and {:?} is {:?}", a.clone(), b.clone(), a.clone().min_try1(b.clone()));

    let c = BigInt::from_vec(vec![1, 2, 3]);
    let d = BigInt::from_vec(vec![3, 2, 1]);
    println!("min of {:?} and {:?} is {:?}", c.clone(), d.clone(), c.clone().min_try1(d.clone()));

    let vbig = vec![a, b, c, d];
    println!("min element of {:?} is {:?}", &vbig, vec_min(&vbig));

    let empty : Vec<i32> = vec![];
    println!("head of {:?}: {:?}", empty, head(&empty));
    let v =vec![4, 2, -3];
    println!("head of {:?}: {:?}", v, head(&v));
}
