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
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let packet = MyPacket::from_bytes(&buffer[0..bytes_read]);
        // Print the received packet
        println!("Received packet: header = {:04x}, payload = {:?}", packet.header, packet.payload);
        // Echo the packet back to the client
        let response = packet.to_bytes();
        stream.write_all(&response)?;
    }
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
