pub use kilos::Kilograms;
pub use name::{Name, BlankNameError};
pub use years::Years;

#[derive(Clone, Debug)]
pub struct Person {
    name: Name,
    age: Years,
    weight: Kilograms,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}, Weight: {}", self.name, self.age, self.weight)
    }
}

mod name {
    use crate::extensions::str_ext::{StrExt, StringExt};

    #[derive(Copy, Clone, Debug)]
    pub struct BlankNameError;

    impl std::fmt::Display for BlankNameError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "A name cannot be blank")
        }
    }

    impl std::error::Error for BlankNameError {}

    #[derive(Clone, Debug)]
    pub struct Name(String);

    impl TryFrom<String> for Name {
        type Error = BlankNameError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.is_not_blank() {
                Ok(Name(value.trim_in_place()))
            } else {
                Err(BlankNameError)
            }
        }
    }

    impl Name {
        pub fn new(value: impl Into<String>) -> Result<Self, BlankNameError> {
            value.into().try_into()
        }
    }

    impl std::fmt::Display for Name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
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
    fn empty_name() {
        assert!(Name::new("").is_err())
    }

    #[test]
    fn blank_name() {
        assert!(Name::new("  ").is_err())
    }

    #[test]
    fn trim_name() {
        assert_eq!("Sherlock", Name::new(" Sherlock ").unwrap().to_string())
    }

    #[test]
    fn display() {
        let sherlock = Person {
            name: Name::new("Sherlock").unwrap(),
            age: Years::new(60),
            weight: Kilograms::new(90),
        };
        println!("{sherlock}");
    }
}

// Ok, so we've moved the code to a common str_ext.rs module.
// Now everybody working on this codebase can leverage these utility functions
// by importing them into scope. And as long as we make sure to keep the functions
// generic it's a net positive!
// ...but wow, we've written a lot of code just to be able to create a Person struct with
// 3 fields and print it out. Ok! let's clean it up a bit and replace some boilerplate with macros
// I'm too lazy to write them all myself so let's use some community crates.