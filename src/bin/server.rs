use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(stream: TcpStream) {
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);

    loop {
        let mut request = String::new();
        reader.read_line(&mut request).unwrap();
        println!("Message: {}", request.trim());
        println!("Send message");
        writer.write("Message received\n".as_bytes()).unwrap();
        writer.flush().unwrap();

        if request.trim() == "EXIT" {
            println!("Closing client connection");
            break;
        }
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        println!("New connection");
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
