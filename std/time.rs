use std::time::{ Duration, SystemTime };

fn main() {
    // Get the current system time
    let now = SystemTime::now();

    // Do some work...

    // Calculate the elapsed time
    if let Ok(elapsed) = now.elapsed() {
        println!("Time elapsed: {:?}", elapsed);
    }
}
