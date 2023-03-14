pub use kilos::Kilograms;
pub use years::Years;

#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    age: Years,
    weight: Kilograms,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Age: {}, Weight: {}",
            self.name, self.age, self.weight
        )
    }
}

mod years {

    #[derive(Copy, Clone, Debug)]
    pub struct Years(u8);

    impl Years {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl std::fmt::Display for Years {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} years", self.0)
        }
    }
}

mod kilos {

    #[derive(Copy, Clone, Debug)]
    pub struct Kilograms(u16);

    impl Kilograms {
        pub fn new(value: u16) -> Self {
            Self(value)
        }
    }

    impl std::fmt::Display for Kilograms {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}kg", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let sherlock = Person {
            name: "Sherlock".to_string(),
            age: Years::new(60),
            weight: Kilograms::new(90),
        };
        assert_eq!(
            "Name: Sherlock, Age: 60 years, Weight: 90kg",
            sherlock.to_string()
        )
    }

    #[test]
    fn display_blank_name() {
        let sherlock = Person {
            name: "".to_string(),
            age: Years::new(60),
            weight: Kilograms::new(90),
        };
        assert_eq!("Name: , Age: 60 years, Weight: 90kg", sherlock.to_string())
    }
}

// Even better, we now clearly enforce the unit of measurement as part of the static types.
// We also can use new types to customize each units display to include the measurement unit type
// The new type pattern is commonly used in rust; a way to offer stronger guarantees of correctness with zero overhead (in most common use-cases)
//
// But wait! A user of our library has opened a bug report!
// Apparently an empty value can be passed in as a name
