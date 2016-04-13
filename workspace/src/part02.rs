// Rust-101, Part 02: Generic types, Traits
// ========================================


// ## Generic datatypes

pub enum SomethingOrNothing<T>  {
    Something(T),
    Nothing,
}
// Instead of writing out all the variants, we can also just import them all at once.
pub use self::SomethingOrNothing::*;
type NumberOrNothing = SomethingOrNothing<i32>;

// ## Generic `impl`, Static functions
// Inside an `impl`, `Self` refers to the type we are implementing things for. Here, it is
// an alias for `SomethingOrNothing<T>`.
impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o {
            None => Nothing,
            Some(t) => Something(t)
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            Nothing => None,
            Something(t) => Some(t)
        }
    }
}
// You can call static functions, and in particular constructors, as demonstrated in `call_constructor`.
fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

// ## Traits

pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            // Here, we can now call the `min` function of the trait.
            Something(n) => {
                e.min(n)
            }
        });
    }
    min
}

// ## Trait implementations
// To make `vec_min` usable with a `Vec<i32>`, we implement the `Minimum` trait for `i32`.
impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl SomethingOrNothing<f32> {
    pub fn print(self) {
        match self {
            Nothing => println!("There is no minimum value"),
            Something(n) => println!("Minimum of f32 vec: {}", n),
        };
    }
}

impl NumberOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,3,9,27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();

    let min_f32 = vec_min(vec![ 42.0, 32.3, 1e19 ]);
    min_f32.print();
}
