#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    age_in_years: u8,
    weight_in_kg: u16,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Age: {} years, Weight: {}kg",
            self.name, self.age_in_years, self.weight_in_kg
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
            age_in_years: 60,
            weight_in_kg: 90,
        };
        assert_eq!(
            "Name: Sherlock, Age: 60 years, Weight: 90kg",
            sherlock.to_string()
        )
    }
}

// Better, our code now explicitly states the unit of measurement.
// But name all our variables *_in_* will get tiresome. Can we do better?
