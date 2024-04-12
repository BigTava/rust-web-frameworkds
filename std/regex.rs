use std::regex::Regex;

fn main() {
    // Create a regular expression pattern
    let pattern = Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap();

    // Test if a string matches the pattern
    let ssn = "123-45-6789";
    if pattern.is_match(ssn) {
        println!("Valid SSN detected!");
    } else {
        println!("Invalid SSN");
    }
}
