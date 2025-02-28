use crate::Command;
use std::io::Error;

struct RESPDataType {}
impl RESPDataType {
    const BULK_STRING: u8 = b'$'; // 0x24
    const ARRAY: u8 = b'*'; // 0x2a
}

// return the offset to skip the parsed data
// If, input = 2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n, len = 0
// then the below function will return (pos=1+2, which is 3), 3 is position of beginning of next field
async fn parse_length(input: &[u8], len: &mut usize) -> usize {
    println!("input: {:?}", std::str::from_utf8(input));
    let mut pos: usize = 0;
    *len = 0;
    while input[pos] != b'\r' {
        *len = *len * 10 + (input[pos] - b'0') as usize;
        pos += 1;
    }
    pos + 2 //return pos+2, which points to the beginning of next field
}

// parse a RESP bulk string from the input byte slice
// check if first byte is a bulk string type
// returns the position which points to the beginning of next field
async fn parse_bulk_string(input: &[u8], result: &mut String) -> Result<usize, Error> {
    if input[0] != RESPDataType::BULK_STRING {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "invalid data"));
    }
    let mut pos: usize = 1;
    let mut string_length = 0;
    pos += parse_length(&input[pos..], &mut string_length).await;
    *result = String::from_utf8_lossy(&input[pos..pos + string_length]).to_string();
    Ok(pos + string_length + 2)
}

async fn parse_array(input: &[u8]) -> Result<Vec<String>, Error> {
    if input[0] != RESPDataType::ARRAY {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "invalid data"));
    }
    let mut pos: usize = 1;
    let mut array_length = 0;
    pos += parse_length(&input[pos..], &mut array_length).await;

    let mut array: Vec<String> = Vec::with_capacity(array_length);
    for _ in 0..array_length {
        let mut arg = String::new();
        pos += parse_bulk_string(&input[pos..], &mut arg).await?;
        array.push(arg);
    }
    Ok(array)
}

pub async fn parse_command(input: &[u8]) -> Result<Command, Error> {
    let tokens = parse_array(input).await?;

    let command = match tokens[0].to_lowercase().as_str() {
        "ping" => Command::Ping,
        "echo" if tokens.len() == 2 => Command::Echo(tokens[1].clone()),
        "set" => match tokens.len() {
            3 => Command::Set(tokens[1].clone(), tokens[2].clone(), None),
            5 if tokens[3].to_lowercase() == "px" => {
                let expiry_in_ms = tokens[4].parse::<u64>().unwrap();
                Command::Set(tokens[1].clone(), tokens[2].clone(), Some(expiry_in_ms))
            }
            _ => Command::Unknown,
        },
        "get" if tokens.len() == 2 => Command::Get(tokens[1].clone()),
        "keys" if tokens.len() == 2 => Command::Keys(tokens[1].clone()),
        "config" => {
            if tokens.len() < 3 {
                return Ok(Command::Unknown);
            }
            match tokens[1].to_lowercase().as_str() {
                "get" => Command::ConfigGet(tokens[2].clone()),
                _ => Command::Unknown,
            }
        }
        _ => Command::Unknown,
    };
    Ok(command)
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_parse_length() {
        let input = b"123\r\n";
        let mut len = 0;
        let pos = parse_length(input, &mut len).await;
        assert_eq!(pos, 5);
        assert_eq!(len, 123);
    }

    #[tokio::test]
    async fn test_parse_bulk_string() {
        let input = b"$3\r\nhey\r\n";
        let mut result = String::new();
        let pos = parse_bulk_string(input, &mut result).await.unwrap();
        assert_eq!(pos, 9);
        assert_eq!(result, "hey");
    }
    #[tokio::test]
    async fn test_parse_array() {
        let input = b"*2\r\n$3\r\nhey\r\n$3\r\nyou\r\n";
        let result = parse_array(input).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "hey");
        assert_eq!(result[1], "you");
    }
}
