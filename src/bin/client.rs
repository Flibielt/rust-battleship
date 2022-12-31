use std::io::{prelude::*, self, BufReader, BufWriter};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878")?;
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);

    let mut message = String::new();
    while message.trim() != "EXIT" {
        let mut response = String::new();
        println!("Write the message:");
        message = "".to_string();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        writer.write(message.as_bytes()).unwrap();
        writer.flush().unwrap();
        reader.read_line(&mut response).unwrap();

        println!("Response: {}", response.trim());
    }

    Ok(())
} // the stream is closed here
