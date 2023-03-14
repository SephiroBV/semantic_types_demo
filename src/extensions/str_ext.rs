pub trait StringExt {
    /// Take an existing `String` and mutate it to remove leading and trailing white space characters.
    fn trim_in_place(self) -> Self;
}

impl StringExt for String {
    fn trim_in_place(mut self) -> Self {
        let trimmed = self.trim();
        let trim_len = trimmed.len();
        if trim_len < self.len() {
            let trim_start = trimmed.as_ptr() as usize - self.as_ptr() as usize;
            if trim_start != 0 {
                self.drain(..trim_start);
            }
            self.truncate(trim_len);
        }
        self
    }
}

pub trait StrExt {
    /// Check if a `str` is non empty after remove leading and trailing white space characters
    fn is_not_blank(&self) -> bool;
}

impl<'a> StrExt for &'a str {
    fn is_not_blank(&self) -> bool {
        !self.trim().is_empty()
    }
}

impl StrExt for String {
    fn is_not_blank(&self) -> bool {
        !self.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(""   ; "when string is empty")]
    #[test_case("   "; "when string is blank")]
    #[test_case("\n" ; "when string contains only new line character")]
    fn is_blank(value: &str) {
        assert!(!value.is_not_blank());
        assert!(!value.to_string().is_not_blank());
    }

    #[test_case("Sherlock"   ; "when string not empty")]
    #[test_case(" Sher Lock "; "when string is not blank but contains whitespaces")]
    fn is_not_blank(value: &str) {
        assert!(value.is_not_blank());
        assert!(value.to_string().is_not_blank());
    }

    #[test_case("Sherlock", "Sherlock"     ; "when string has no leading or trailing spaces")]
    #[test_case(" Sherlock", "Sherlock"    ; "when string has leading spaces")]
    #[test_case("Sherlock ", "Sherlock"    ; "when string has trailing spaces")]
    #[test_case(" Sherlock ", "Sherlock"   ; "when string has leading and trailing spaces")]
    #[test_case(" Sher lock ", "Sher lock" ; "when string has leading and trailing spaces and spaces in middle of name")]
    fn trim_in_place(input: &str, output: &str) {
        assert_eq!(output, input.to_string().trim_in_place());
    }
}
