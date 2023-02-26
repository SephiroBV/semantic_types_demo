#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    age: u8,
    weight: u16,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}, Weight: {}", self.name, self.age, self.weight)
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
        println!("{sherlock}");
    }
}

// A basic start, a person has a name, age and weight.
// We can probably guess that the unit of measurement for age is years
// But what about weight? Is it kilograms or pounds? Let's be explicit!