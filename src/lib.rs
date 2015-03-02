pub enum Fx {
    Fizz,
    Buzz,
    FizzBuzz,
    I(u32),
}

impl Fx {
    pub fn new(n: u32) -> Fx {
        Fx::arbitrary(n, |n| n % 3 == 0, |n| n % 5 == 0)
    }

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
