use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{Shutdown, TcpStream};

const SERVER: &'static str = "127.0.0.1:8080";
const REQUEST_PATH: &'static str = "/test.html";

fn main() -> std::io::Result<()> {
    println!("Server: {:?}", SERVER);
    let stream = TcpStream::connect(SERVER)?;

    println!("request: {:?}", REQUEST_PATH);
    let mut writer = BufWriter::new(&stream);
    writer.write(format!("{}\n", REQUEST_PATH).as_bytes())?;
    writer.flush()?;

    println!("read responses...");
    let mut response = String::new();
    let mut reader = BufReader::new(&stream);
    reader.read_line(&mut response)?;

    println!("response: {:?}", response);
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}
