use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};

// Enum to represent command names
#[derive(Debug)]
enum CommandName {
    Ping,
    Echo,
    Set,
    Get,
}

// Struct to represent a command
#[derive(Debug)]
struct Command {
    name: CommandName,
    args: Vec<String>,
}

impl Command {
    // Function to generate a response based on the command
    fn generate_response(&self, db: &mut Database) -> String {
        match &self.name {
            CommandName::Ping => "+PONG\r\n".to_string(),
            CommandName::Echo => format!("+{}\r\n", self.args[0]),
            CommandName::Set => {
                // for command - "*3\r\n$3\r\nset\r\n$7\r\noranges\r\n$5\r\nworld\r\n", args will have - "$7", "oranges", "$5", "world"
                if self.args.len() == 4 {
                    db.set(&self.args[1], &self.args[3]);
                    "+OK\r\n".to_string()
                } else {
                    "-ERR wrong number of argument for 'SET' command\r\n".to_string()
                }
            }
            CommandName::Get => {
                // for command - "*3\r\n$3\r\nget\r\n$7\r\noranges\r\n", args will have - "$7", "oranges"
                if let Some(value) = db.get(&self.args[1]) {
                    format!("+{}\r\n", value)
                } else {
                    "$-1\r\n".to_string() // Null bulk reply for non-existing key
                }
            }
        }
    }
}

// Database struct to store key-value pairs
#[derive(Clone)]
struct Database {
    db: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database { db: HashMap::new() }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.db.get(key)
    }

    fn set(&mut self, key: &str, value: &str) {
        self.db.insert(key.to_owned(), value.to_owned());
    }
}

// Function to parse the request and return a Command
fn parse_request(request: &str) -> Command {
    let parts: Vec<&str> = request.trim().split_whitespace().collect();

    // Check if it's a PING, ECHO, SET, or GET command
    match parts.get(2).map(|&s| s.to_uppercase()) {
        Some(s) => {
            if s == "PING" {
                Command {
                    name: CommandName::Ping,
                    args: Vec::new(),
                }
            } else if s == "ECHO" {
                let message = parts[4];
                Command {
                    name: CommandName::Echo,
                    args: vec![message.to_string()],
                }
            } else if s == "SET" {
                Command {
                    name: CommandName::Set,
                    args: parts[3..].into_iter().map(|s| s.to_string()).collect(),
                }
            } else if s == "GET" {
                Command {
                    name: CommandName::Get,
                    args: parts[3..].into_iter().map(|s| s.to_string()).collect(),
                }
            } else {
                panic!("Unknown command format");
            }
        }
        None => panic!("Invalid command format"),
    }
}

// Function to handle incoming connections
fn handle_connection(mut tcp_stream: TcpStream, db: &mut Database) {
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
                let response = command.generate_response(db);

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

    // Create a new Database instance
    let db = Database::new();

    // this for loop is for accepting incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                println!("accepted new connection");

                // Clone the db value before passing it to the closure
                let mut cloned_db = db.clone();

                // Spawn a new thread for each incoming connection
                // The move keyword is used to transfer ownership of the TcpStream to the spawned thread.
                thread::spawn(move || {
                    handle_connection(tcp_stream, &mut cloned_db);
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
