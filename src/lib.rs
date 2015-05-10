//! FizzBuzz helper library for Rust and Cargo
//!
//! Instructions: Add to your cargo tomato and go!
//!
//! # Example 1: Basic fizzbuzz
//!
//! ```
//! use fx::Fx;
//!
//! for i in (1..101).map(fx::default) {
//!     println!("{}", i);
//! }
//! ```
//!
//! # Example 2: Sum of inverse
//!
//! ```
//! use fx::Fx;
//!
//! let sum_of_non_fizz_buzz_values = (1..101)
//!     .map(fx::default)
//!     .fold(0, |a,b| a + if let Fx::I(n) = b {
//!         n
//!     } else {
//!         0
//!     });
//!
//! assert!(sum_of_non_fizz_buzz_values == 2632);
//! ```
//!
//! # Macro use
//!
//! The fx! macro takes two forms. The first accepts Rust's range notation, e.g. 1..101 for values
//! 1 through 100 (the upper bound is exclusive in n..m range notation). The second instead accepts
//! inclusive lower and upper bounds, e.g. 1 and 100 for values 1 through 100.
//!
//! > fx!(1..101);
//! > fx!(1, 100);
//!
//! I mostly did this because it's kind of neat to me that macros wind up serving as a form of poor
//! man's function overloading. >.>

#[macro_export]
macro_rules! fx {
    ($range:expr) => {
        for i in ($range).map(fx::default) {
            println!("{}", i);
        }
    };

    ($n:expr, $m:expr) => {
        for i in ($n..($m + 1)).map(fx::default) {
            println!("{}", i);
        }
    };
}

/// Enum representing value status.
pub enum Fx {
    Fizz(u32),
    Buzz(u32),
    FizzBuzz(u32),
    I(u32),
}

impl Fx {
    /// Create new Fx enum using standard FizzBuzz rules.
    pub fn new(n: u32) -> Fx {
        Fx::arbitrary(n, |n| n % 3 == 0, |n| n % 5 == 0)
    }

    /// Create new Fx enum using arbitrary FizzBuzz rules.
    pub fn arbitrary<FA, FB>(n: u32, fa: FA, fb: FB) -> Fx
        where FA: Fn(u32) -> bool,
              FB: Fn(u32) -> bool
    {
        match (n, fa(n), fb(n)) {
            (n, true, true) => Fx::FizzBuzz(n),
            (n, true, _) => Fx::Fizz(n),
            (n, _, true) => Fx::Buzz(n),
            (n, _, _) => Fx::I(n),
        }
    }
}

/// Print Fx value to console.
impl std::fmt::Display for Fx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Fx::FizzBuzz(_) => write!(f, "FizzBuzz"),
            &Fx::Fizz(_) => write!(f, "Fizz"),
            &Fx::Buzz(_) => write!(f, "Buzz"),
            &Fx::I(n) => write!(f, "{}", n),
        }
    }
}

/// Convenience method for creating a new default Fx enum.
pub fn default(n: u32) -> Fx {
    Fx::new(n)
}

#[cfg(test)]
mod test {
    use super::Fx;

    #[test]
    fn integers_work() {
        assert!(format!("{}", Fx::new(2)) == "2");
    }

    #[test]
    fn fizz_works() {
        assert!(format!("{}", Fx::new(3)) == "Fizz");
    }

    #[test]
    fn buzz_works() {
        assert!(format!("{}", Fx::new(5)) == "Buzz");
    }

    #[test]
    fn fizz_buzz_works() {
        assert!(format!("{}", Fx::new(15)) == "FizzBuzz");
    }
}
