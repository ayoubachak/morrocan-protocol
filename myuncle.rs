use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct MyPacket {
    header: u16,
    payload: Vec<u8>,
}

impl MyPacket {
    fn from_bytes(bytes: &[u8]) -> MyPacket {
        let header = u16::from_be_bytes([bytes[0], bytes[1]]);
        let payload = bytes[2..].to_vec();
        MyPacket { header, payload }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(2 + self.payload.len());
        bytes.extend_from_slice(&self.header.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Client connected!");

    // Conversation between client and server
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"Hey man how are you doing ?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"I'm fine thank you, how are you doing ?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"How is life ?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"I'm fine !\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"How are you man!\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"I swear I'm FINE!\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"How's everything going?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b":<\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"How have you been lately?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b"How are you feeling today?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Me: {}", String::from_utf8_lossy(&buf));
    stream.write(b":<\n").unwrap();

    // Close the connection
    stream.shutdown(std::net::Shutdown::Both).unwrap();
    println!("Connection closed!");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    println!("Listening on {}", listener.local_addr()?);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match handle_client(stream) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error handling client: {}", e),
                }
            }
            Err(e) => eprintln!("Error accepting client connection: {}", e),
        }
    }
    Ok(())
}
