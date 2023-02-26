pub use kilos::Kilograms;
pub use name::{Name, BlankNameError};
pub use years::Years;

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "Name: {}, Age: {}, Weight: {}", name, age, weight)]
pub struct Person {
    name: Name,
    age: Years,
    weight: Kilograms,
}

mod name {
    use crate::extensions::str_ext::{StrExt, StringExt};

    #[derive(Copy, Clone, Debug, thiserror::Error, derive_more::Display)]
    #[display(fmt = "A name cannot be blank")]
    pub struct BlankNameError;

    #[derive(Clone, Debug, derive_more::Display)]
    pub struct Name(String);

    impl TryFrom<String> for Name {
        type Error = BlankNameError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.is_not_blank()
                .then(|| value.trim_in_place())
                .map(|inner| Name(inner))
                .ok_or(BlankNameError)
        }
    }

    impl Name {
        pub fn new(value: impl Into<String>) -> Result<Self, BlankNameError> {
            value.into().try_into()
        }
    }

}

mod years {

    #[derive(Copy, Clone, Debug, derive_more::Display, derive_more::Constructor)]
    #[display(fmt = "{} years", _0)]
    pub struct Years(u8);
}

mod kilos {

    #[derive(Copy, Clone, Debug, derive_more::Display, derive_more::Constructor)]
    #[display(fmt = "{}kg", _0)]
    pub struct Kilograms(u16);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(""   ; "when name is empty")]
    #[test_case("   "; "when name is blank")]
    fn new_name_error(inner: &str) {
        assert!(Name::new(inner).is_err())
    }

    #[test_case("Sherlock", "Sherlock"     ; "when string has no leading or trailing spaces")]
    #[test_case(" Sherlock", "Sherlock"    ; "when string has leading spaces")]
    #[test_case("Sherlock ", "Sherlock"   ; "when string has trailing spaces")]
    #[test_case(" Sherlock ", "Sherlock"   ; "when string has leading and trailing spaces")]
    #[test_case(" Sher lock ", "Sher lock" ; "when string has leading and trailing spaces and spaces in middle of name")]
    fn trim_name(input: &str, output: &str) {
        assert_eq!(output, Name::new(input).unwrap().to_string());
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

// Ok, so we shaved some lines off the code, nice!
// But this example is just the beginning. There are all sort of
// domain specific functions you can implement on your semantic types!

// You can extend this example yourself! For example, what does Name mean exactly?
// Is it someone's first name or last name or full name? We can be more explicit