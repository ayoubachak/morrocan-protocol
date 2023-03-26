use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    println!("Connected to server!");

    // Conversation between client and server
    stream.write(b"Hello!\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"I'm fine thank you, how are you doing ?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"How is life ?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"I'm fine !\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"How are you man!\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"I swear I'm FINE!\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"How's everything going?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b":<\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"How have you been lately?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b"How are you feeling today?\n").unwrap();

    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("My Uncle: {}", String::from_utf8_lossy(&buf));

    stream.write(b":<\n").unwrap();

    // Close the connection
    stream.shutdown(std::net::Shutdown::Both).unwrap();
    println!("Connection closed!");
    Ok(())
}
