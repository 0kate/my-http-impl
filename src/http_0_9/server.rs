use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{Shutdown, TcpListener};

const PUBLIC_DIR: &'static str = "/tmp";

fn main() -> std::io::Result<()> {
    // TCPソケットを作成して8080ポートにバインド
    let server = TcpListener::bind("127.0.0.1:8080")?;

    let (client, addr) = server.accept()?;
    println!("connected: {:?}", addr);

    let mut request = String::new();
    let mut reader = BufReader::new(&client);
    reader.read_line(&mut request)?;

    println!("request: {:?}", request);

    let mut file = File::open(format!("{}{}", PUBLIC_DIR, request.trim()))?;
    let mut response = String::new();
    file.read_to_string(&mut response)?;

    println!("response: {:?}", response);
    let mut writer = BufWriter::new(&client);
    writer.write(response.as_bytes())?;
    writer.flush()?;

    client.shutdown(Shutdown::Both)?;

    Ok(())
}
