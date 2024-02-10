mod parse;
mod store;
use std::io::Error;
use store::Database;

use parse::parse_command;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
};

pub enum Command {
    Ping,
    Echo(String),
    Set(String, String, Option<u64>),
    Get(String),
    ConfigGet(String),
    Keys(String),
    Unknown,
}

async fn execute_command(
    stream: &mut TcpStream,
    command: Command,
    db: &Database,
) -> Result<(), Error> {
    let resp: String = match command {
        Command::Ping => "+PONG\r\n".to_string(),

        Command::Echo(echo_arg) => {
            format!("+{}\r\n", echo_arg)
        }
        Command::Set(key, value, expiry_in_ms) => match expiry_in_ms {
            Some(expiry_in_ms) => {
                db.set_with_expire(&key, &value, expiry_in_ms).await;
                "+OK\r\n".to_string()
            }
            None => {
                db.set(&key, &value).await;
                "+OK\r\n".to_string()
            }
        },
        Command::Get(key) => match db.get(&key).await {
            Some(value) => {
                format!("+{}\r\n", value)
            }
            None => "$-1\r\n".to_string(),
        },
        Command::Keys(pattern) => {
            let mut keys = db.keys(&pattern).await;
            keys.sort();
            let mut resp = String::new();
            resp.push_str(&format!("*{}\r\n", keys.len()));
            for key in keys {
                resp.push_str(&format!("${}\r\n{}\r\n", key.len(), key));
            }
            resp
        }
        Command::ConfigGet(key) => match db.config_get(key.as_str()).await {
            Some(value) => {
                format!(
                    "*2\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
                    key.len(),
                    key,
                    value.len(),
                    value
                )
            }
            None => "$-1\r\n".to_string(),
        },
        Command::Unknown => "-ERR unknown command\r\n".to_string(),
    };
    stream.write_all(resp.as_bytes()).await?;
    Ok(())
}

async fn handle_stream(stream: TcpStream, db: &Database) -> Result<(), Error> {
    let mut stream = stream;
    let mut buf = [0; 1024];
    while let Ok(n) = stream.read(&mut buf).await {
        if n == 0 {
            break;
        }
        match parse_command(&buf[..n]).await {
            Ok(cmd) => execute_command(&mut stream, cmd, db).await?,
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here! ");

    let db = Database::new();
    let db = Arc::new(db);

    // The below line creates a TCP listener bound to the address "127.0.0.1" (localhost) and port 6379.
    // The unwrap() method is used to handle the Result returned by bind,
    // and it will panic and exit if the binding fails.
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Failed to bind");

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((_stream, _addr)) => {
                println!("accepted new connection");
                let db = Arc::clone(&db);
                spawn(async move {
                    if let Err(e) = handle_stream(_stream, &db).await {
                        println!("error: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
