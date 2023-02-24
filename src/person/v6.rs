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