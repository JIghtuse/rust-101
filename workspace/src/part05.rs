// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.is_empty() {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    // We can convert any vector of digits into a number, by removing trailing zeros. The `mut`
    // declaration for `v` here is just like the one in `let mut ...`: We completely own `v`, but Rust
    // still asks us to make our intention of modifying it explicit. This `mut` is *not* part of the
    // type of `from_vec` - the caller has to give up ownership of `v` anyway, so they don't care anymore
    // what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        while v.len() > 0 && v[v.len() - 1] == 0 {
            v.pop();
        }
        BigInt { data: v }
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v.clone());
    let b3 = BigInt::from_vec(v);
    assert!(b1.test_invariant());
    assert!(b2.test_invariant());
    assert!(b3.test_invariant());
}

// We can also make the type `SomethingOrNothing<T>` implement `Clone`. 
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone()),
        }
    }
}

impl BigInt {
    fn number_of_digits(&self) -> usize {
        self.data.len()
    }
    fn number_of_nonzero_digits(&self) -> usize {
        self.data.iter().filter(|&x| *x != 0).count()
    }
    fn smallest_digit(&self) -> Option<u64> {
        self.data.iter().min().cloned()
    }
    fn largest_digit(&self) -> Option<u64> {
        self.data.iter().max().cloned()
    }
}

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}

pub fn main() {
    clone_demo();

    let empty = BigInt::new(0);
    assert!(empty.test_invariant());
    assert!(empty.data == []);
    assert!(empty.number_of_digits() == 0);
    assert!(empty.number_of_nonzero_digits() == 0);
    assert!(empty.smallest_digit().is_none());
    assert!(empty.largest_digit().is_none());

    let b = BigInt::from_vec(vec![0, 3, 2, 4, 5, 0]);
    assert!(b.test_invariant());
    assert!(b.data == [0, 5, 4, 2, 3]);
    assert!(b.number_of_digits() == 5);
    assert!(b.number_of_nonzero_digits() == 4);
    assert!(b.smallest_digit() == Some(0));
    assert!(b.largest_digit() == Some(5));
}
