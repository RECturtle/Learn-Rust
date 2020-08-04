pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Will throw the custom error message below
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_two_three() {
        assert_ne!(4, add_two(3));
    }
    
    // Custom error message
    #[test]
    #[should_panic] // expected panic test
    #[ignore] // will ignore this test until specifically called
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }
}
