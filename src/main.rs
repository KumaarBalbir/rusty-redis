use std::{
    io::{Read, Write},
    net::TcpListener,
    str,
};
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // The below line creates a TCP listener bound to the address "127.0.0.1" (localhost) and port 6379.
    // The unwrap() method is used to handle the Result returned by bind,
    // and it will panic and exit if the binding fails.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    //The for stream in listener.incoming() loop iterates over incoming connections.
    // The incoming method returns an iterator over incoming connections.
    for stream in listener.incoming() {
        // match the stream with the Result type
        match stream {
            Ok(_stream) => {
                // When a connection is successfully established
                // _stream is a placeholder indicating that the
                // stream is accepted but not explicitly used in this example
                println!("accepted new connection");
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
