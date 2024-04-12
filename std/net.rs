use std::net::{ TcpListener, TcpStream };

fn main() {
    // Set up a TCP listener
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle the connection
                println!("Accepted connection from: {}", stream.peer_addr().unwrap());
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
