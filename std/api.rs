use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on 127.0.0.1:8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }
    let request = String::from_utf8_lossy(&buffer[..]);

    if request.contains("GET /api") {
        let response =
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, Rust API!\"}";

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to connection: {}", e);
            return;
        }
    } else {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot Found";
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to connection: {}", e);
            return;
        }
    }

    // Flush the output stream
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}
