struct Greeter {
    noun: String,
}

impl Greeter {
    fn new(noun: &str) -> Self {
        Greeter { noun: noun.into() }
    }

    fn greet(&self) -> String {
        format!("Hello {}!", self.noun)
    }

    fn saluer(&self) -> String {
        format!("Bonjour {}!", self.noun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english() {
        let greeter = Greeter::new("World");
        assert_eq!(greeter.greet(), "Hello World!");
    }

    #[test]
    fn french() {
        let greeter = Greeter::new("Monde");
        assert_eq!(greeter.saluer(), "Bonjour Monde!");
    }
}
