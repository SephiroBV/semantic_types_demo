pub use kilos::Kilograms;
pub use name::Name;
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

        fn try_from(mut value: String) -> Result<Self, Self::Error> {
            if is_not_blank(&value) {
                // let's trim any extra leading or trailing white spaces if required
                trim_in_place(&mut value);
                Ok(Name(value))
            } else {
                // If input is blank, return an error
                Err(BlankNameError)
            }
        }
    }

    fn is_not_blank(value: &str) -> bool {
        !value.trim().is_empty()
    }

    fn trim_in_place(value: &mut String) {
        let trimmed = value.trim();
        let trim_len  = trimmed.len();
        if trim_len < value.len() {
            let trim_start= trimmed.as_ptr() as usize - value.as_ptr() as usize;
            if trim_start !=0 {
                value.drain(..trim_start);
            }
            value.truncate(trim_len);
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

// Ok, we've solved the blank name problem!
// Don't believe me? Check the tests above.
// But something seems a little out of place.
// Those string helper functions we added, they seem pretty useful.
// We should probably make them readily available to the rest of our codebase
// Let's not leave them buried in some random module