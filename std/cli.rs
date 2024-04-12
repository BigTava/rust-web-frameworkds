use std::env;

fn main() {
    // Access command line arguments
    let args: Vec<String> = env::args().collect();

    // Print the program name and arguments
    println!("Program: {}", args[0]);
    println!("Arguments: {:?}", &args[1..]);
}
