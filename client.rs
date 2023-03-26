use std::io::{Read, Write};
use std::net::TcpStream;

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
        bytes.extend_from_slice(&self.payload.as_slice());
        bytes
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    let packet = MyPacket { header: 0x1234, payload: vec![0x01, 0x02, 0x03] };
    let request = packet.to_bytes();
    stream.write_all(&request)?;
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = MyPacket::from_bytes(&buffer[0..bytes_read]);
    println!("Received response: header = {:04x}, payload = {:?}", response.header, response.payload);
    Ok(())
}
