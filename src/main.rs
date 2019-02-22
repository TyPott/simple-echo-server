use std::env;
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, TcpListener};
use std::num::NonZeroU64;
use std::time::Duration;

const BUF_SIZE: usize = 8 * 1024;
const TIMEOUT: u64 = 15;

fn echo<T: Read + Write>(client: &mut T) -> io::Result<usize> {
    let mut buffer = [0; BUF_SIZE];
    let mut written = 0;
    loop {
        let n = client.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        client.write_all(&buffer[0..n])?;
        written += n;
    }
    Ok(written)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<usize> {
    let time = NonZeroU64::new(TIMEOUT).map(|n| Duration::from_secs(n.get()));
    stream.set_read_timeout(time)?;
    stream.set_write_timeout(time)?;
    echo(stream)
}

fn run_server(listener: TcpListener) {
    println!("Server listening on {:?}", listener);
    for client in listener.incoming() {
        match client.and_then(|mut s| handle_client(&mut s)) {
            Ok(n) => println!("Wrote {} bytes", n),
            Err(e) => println!("Error reading/writing stream: {}", e),
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let addr = args
        .next()
        .and_then(|a| a.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = args.next().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let socket = SocketAddr::new(addr, port);

    match TcpListener::bind(socket) {
        Ok(listener) => run_server(listener),
        Err(e) => println!("Failed to bind to socket: {}", e),
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
        assert_eq!(input.len(), echo(&mut buffer).unwrap());
        assert_eq!(input.repeat(2).as_bytes(), buffer.get_ref().as_slice());
    }
}
