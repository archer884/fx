//! FizzBuzz helper library for Rust and Cargo
//!
//! Instructions: Add to your cargo tomato and go!
//!
//! # Example
//!
//! ```
//! use fx::Fx;
//!
//! let sum_of_non_fizz_buzz_values = (1..101)
//!     .map(default)
//!     .fold(0, |a,b| a + if let I(n) = b {
//!         n
//!     } else {
//!         0
//!     });
//!
//! assert!(sum_of_nfizz_buzz_values == 2632);
//! ```

/// Enum representing value status.
pub enum Fx {
    Fizz,
    Buzz,
    FizzBuzz,
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
            (_, true, true) => Fx::FizzBuzz,
            (_, true, _) => Fx::Fizz,
            (_, _, true) => Fx::Buzz,
            (n, _, _) => Fx::I(n),
        }
    }
}

/// Print Fx value to console; no need to futz with formatting.
/// on the client side.
impl std::fmt::Display for Fx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Fx::FizzBuzz => write!(f, "FizzBuzz"),
            &Fx::Fizz => write!(f, "Fizz"),
            &Fx::Buzz => write!(f, "Buzz"),
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
