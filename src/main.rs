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

    //a buffer is created to store incoming data from the network stream.
    //The buffer is an array of 1024 bytes(u8 elements), initialized to zero.
    //This buffer will be used to read data sent by clients to the server.
    //When a client sends a command, the server will read the bytes into this buffer to process the command.
    //The mut keyword indicates that the contents of buffer can change,
    //which is necessary because it will be filled with the data read from the network stream.
    let mut buffer: [u8; 1024] = [0; 1024];

    //The for stream in listener.incoming() loop iterates over incoming connections.
    // The incoming method returns an iterator over incoming connections.
    for stream in listener.incoming() {
        // match the stream with the Result type
        match stream {
            Ok(mut tcp_stream) => {
                // When a connection is successfully established
                // _stream is a placeholder indicating that the
                // stream is accepted but not explicitly used in this example
                println!("accepted new connection");
                // read method for tcp stream, pass buffer as arg to fill it with
                // incoming data. num_bytes stores the number of bytes read
                let res = tcp_stream.read(&mut buffer);
                // match statement to handle the result of read operation
                match res {
                    Ok(num_bytes) => {
                        println!("read {} bytes", num_bytes);
                        println!("data: {:?}", str::from_utf8(&buffer[..num_bytes]));

                        //check if the received data matches the encoded PING command,
                        //if it does server responds with the encoded +PONG\r\n message using write method.
                        if buffer[..num_bytes] == b"*1\r\n$4\r\nPING\r\n"[..num_bytes]
                            || buffer[..num_bytes] == b"*1\r\n$4\r\nping\r\n"[..num_bytes]
                        {
                            let _res_write = tcp_stream.write(b"+PONG\r\n");
                        }
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
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
