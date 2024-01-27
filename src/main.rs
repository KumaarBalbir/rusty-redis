use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};

// Enum to represent command names
#[derive(Debug)]
enum CommandName {
    Ping,
    Echo,
}

// Struct to represent a command
#[derive(Debug)]
struct Command {
    name: CommandName,
    args: Vec<String>,
}

impl Command {
    // Function to generate a response based on the command
    fn generate_response(&self) -> String {
        match &self.name {
            CommandName::Ping => "+PONG\r\n".to_string(),
            CommandName::Echo => format!("+{}\r\n", self.args[0]),
        }
    }
}

// Function to parse the request and return a Command
fn parse_request(request: &str) -> Command {
    let parts: Vec<&str> = request.trim().split_whitespace().collect();

    // Check if it's a PING command
    if parts[2] == "PING" || parts[2] == "ping" {
        Command {
            name: CommandName::Ping,
            args: Vec::new(),
        }
    } else if parts[2] == "ECHO" || parts[2] == "echo" {
        // Check if it's an ECHO command
        let _length: usize = parts[3].parse().unwrap();
        let message = parts[4];
        Command {
            name: CommandName::Echo,
            args: vec![message.to_string()],
        }
    } else {
        panic!("Unknown command format");
    }
}

// Function to handle incoming connections
fn handle_connection(mut tcp_stream: TcpStream) {
    // Create a loop to continuously process incoming data on the same connection
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];

        match tcp_stream.read(&mut buffer) {
            Ok(num_bytes) => {
                if num_bytes == 0 {
                    // Connection closed by the client
                    println!("Connection closed by client");
                    break;
                }

                println!("read {} bytes", num_bytes);
                println!("data: {:?}", str::from_utf8(&buffer[..num_bytes]));

                // parse the request
                let request = str::from_utf8(&buffer[..num_bytes]).unwrap();
                let command = parse_request(request);

                // Generate a response based on the command
                let response = command.generate_response();

                // Respond to the client
                let _res_write = tcp_stream.write(response.as_bytes());

                // Check if the received data matches the encoded PING command,
                // and respond with the hardcoded +PONG\r\n message using write method.

                // if buffer[..num_bytes] == b"*1\r\n$4\r\nPING\r\n"[..num_bytes]
                //     || buffer[..num_bytes] == b"*1\r\n$4\r\nping\r\n"[..num_bytes]
                // {
                //     let _res_write = tcp_stream.write(b"+PONG\r\n");
                // }
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // The below line creates a TCP listener bound to the address "127.0.0.1" (localhost) and port 6379.
    // The unwrap() method is used to handle the Result returned by bind,
    // and it will panic and exit if the binding fails.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // this for loop is for accepting incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                println!("accepted new connection");

                // Spawn a new thread for each incoming connection
                // The move keyword is used to transfer ownership of the TcpStream to the spawned thread.
                thread::spawn(move || {
                    handle_connection(tcp_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// Note:
// This code represents the basic structure of a TCP server.
// However, it's important to note that this example
// does not handle the accepted connection beyond printing a message.
// In a complete application, you would typically spawn a new thread
// or use an asynchronous runtime to handle each accepted connection concurrently.
// Additionally, handling of incoming data, parsing messages, and
// responding to clients would be implemented.
