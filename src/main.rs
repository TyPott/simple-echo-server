use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;

const HOST: &str = "127.0.0.1:8080";
const BUF_SIZE: usize = 8 * 1024;

fn echo<T: Read + Write>(client: &mut T) -> io::Result<()> {
    let mut buffer = [0; BUF_SIZE];
    loop {
        let n = client.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        client.write_all(&buffer[0..n])?;
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(15)))?;
    stream.set_write_timeout(Some(Duration::from_secs(15)))?;
    echo(stream)
}

fn main() {
    let listener = TcpListener::bind(HOST).expect("Failed to bind socket");
    println!("Server listening on {}", HOST);

    for client in listener.incoming() {
        if let Err(e) = client.and_then(|mut s| handle_client(&mut s)) {
            println!("Error reading/writing stream: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::Cursor;

    #[test]
    fn echo_in_memory_buffer() {
        let input = "Hello, world!";
        let mut buffer = Cursor::new(input.to_owned().into_bytes());
        assert_eq!((), echo(&mut buffer).unwrap());
        assert_eq!(input.repeat(2).as_bytes(), buffer.get_ref().as_slice());
    }
}
