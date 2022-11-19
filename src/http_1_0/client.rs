use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader};
use std::net::TcpStream;

const SERVER: &'static str = "127.0.0.1:8080";
const REQUEST_PATH: &'static str = "/test.html";
const USER_AGENT: &'static str = "my-http-1.0";

fn parse_headline(headline: &String) -> (&str, &str, &str) {
    let mut iter = headline.splitn(3, ' ');
    let version = iter.next().unwrap();
    let status = iter.next().unwrap();
    let message = iter.next().unwrap();

    (version, status, message)
}

fn parse_headers(header_lines: &Vec<String>, headers: &mut HashMap<String, String>) {
    for header_line in header_lines.iter() {
        if let Some((key, value)) = header_line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(SERVER)?;
    stream.write(format!("GET {} HTTP/1.0\r\n", REQUEST_PATH).as_bytes())?;
    stream.write(format!("User-Agent: {}\r\n", USER_AGENT).as_bytes())?;
    stream.write(b"Accept: text/html\r\n")?;
    stream.write(b"\r\n")?;
    stream.flush()?;

    let mut reader = BufReader::new(&stream);

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let (version, status, message) = parse_headline(&buffer);

    let mut buffer = String::new();
    let mut header_lines = Vec::new();
    loop {
        reader.read_line(&mut buffer)?;
        if buffer.trim().is_empty() {
            break;
        }

        header_lines.push(buffer.clone());
        buffer.clear();
    }

    let mut headers = HashMap::new();
    parse_headers(&header_lines, &mut headers);

    let mut body = String::new();
    reader.read_to_string(&mut body);

    println!("{:?}", body);

    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
