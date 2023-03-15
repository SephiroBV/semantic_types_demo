type Kilograms = u16;
type Years = u8;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let sherlock = Person {
            name: "Sherlock".to_string(),
            age: 60,
            weight: 90,
        };
        assert_eq!("Name: Sherlock, Age: 60, Weight: 90", sherlock.to_string())
    }
}

// Type aliases!
// We can use them to refer to primitives and other generic data types with domain specific naming
// This makes the code more readable, but lacks stronger guarantees on correctness.
// Let's go further
