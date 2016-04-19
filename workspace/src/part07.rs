// Rust-101, Part 07: Operator Overloading, Tests, Formatting
// ==========================================================

pub use part05::BigInt;

// With our new knowledge of lifetimes, we are now able to write down the desired type of `min`:
pub trait Minimum {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min: Option<&T> = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => n.min(e)
        });
    }
    min
}

impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
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

// ## Operator Overloading

impl PartialEq for BigInt {
    #[inline]
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        self.data == other.data
    }
}


// Now we can compare `BigInt`s. Rust treats `PartialEq` special in that it is wired to the operator `==`:
fn compare_big_ints() {
    let b1 = BigInt::new(13);
    let b2 = BigInt::new(37);
    println!("b1 == b1: {} ; b1 == b2: {}; b1 != b2: {}", b1 == b1, b1 == b2, b1 != b2);
}

// ## Testing
// With our equality test written, we are now ready to write our first testcase.
// the `test` attribute. `assert!` is like `debug_assert!`, but does not get compiled away in a release build.
#[test]
fn test_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    assert!(b1.min(&b2) == &b1);
    assert!(b2.min(&b1) == &b1);
    assert!(b1.min(&b3) == &b1);
}
// Now run `cargo test` to execute the test. If you implemented `min` correctly, it should all work!

// ## Formatting

// All formating is handled by [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html). I won't explain
// all the details, and refer you to the documentation instead.
use std::fmt;

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

// Now we are ready to use `assert_eq!` to test `vec_min`.
#[test]
fn test_vec_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    let v1 = vec![b2.clone(), b1.clone(), b3.clone()];
    let v2 = vec![b2.clone(), b3.clone()];
    assert_eq!(Some(&b1), vec_min(&v1));
    assert_eq!(Some(&b2), vec_min(&v2));

    let empty : Vec<BigInt> = vec![];
    assert_eq!(None, vec_min(&empty));
}

#[test]
fn test_from_vec() {
    let b1 = BigInt::from_vec(vec![1, 1]);
    assert_eq!(vec![1, 1], b1.data);
    let b2 = BigInt::from_vec(vec![1, 0]);
    assert_eq!(vec![1], b2.data);
    let b2 = BigInt::from_vec(vec![0, 0, 0]);
    assert_eq!(Vec::<u64>::new(), b2.data);
}

pub fn main() {
    let a = BigInt::from_vec(vec![1]);
    let b = BigInt::from_vec(vec![3]);
    println!("min of {:?} and {:?} is {:?}", a, b, a.min(&b));

    let c = BigInt::from_vec(vec![1, 2, 3]);
    let d = BigInt::from_vec(vec![3, 2, 1]);
    println!("min of {:?} and {:?} is {:?}", c, d, c.min(&d));

    let vbig = vec![a, b, c, d];
    println!("min element of {:?} is {:?}", &vbig, vec_min(&vbig));
}
