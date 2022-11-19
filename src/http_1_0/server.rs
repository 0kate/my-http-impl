use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

const PUBLIC_DIR: &'static str = "/tmp";

fn parse_headline(headline: &String) -> (&str, &str, &str) {
    let mut iter = headline.splitn(3, ' ');

    let method = iter.next().unwrap();
    let path = iter.next().unwrap();
    let version = iter.next().unwrap().trim();

    (method, path, version)
}

fn parse_headers(header_lines: &Vec<String>, headers: &mut HashMap<String, String>) {
    for header_line in header_lines {
        if let Some((key, value)) = header_line.split_once(": ") {
            headers.insert(key.to_string(), value.trim().to_string());
        }
    }
}

fn handle_get(client: &mut TcpStream, path: &str, headers: &HashMap<String, String>, body: &String) -> std::io::Result<()> {
    let (status, contents) = match File::open(format!("{}{}", PUBLIC_DIR, path)) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            ("200 OK", contents)
        },
        Err(e) => ("404 Not Found", String::new()),
    };
    println!("contents: {:?}", contents);

    writeln!(client, "HTTP/1.0 {}", status)?;
    writeln!(client, "Content-Length: {}", contents.len())?;
    writeln!(client, "Content-Type: text/html")?;
    writeln!(client)?;
    writeln!(client, "{}", contents)?;

    Ok(())
}

fn handle_post(client: &mut TcpStream, path: &str, headers: &HashMap<String, String>, body: &String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}{}", PUBLIC_DIR, path))?;
    file.write_all(body.as_bytes())?;

    writeln!(client, "HTTP/1.0 200 OK")?;
    writeln!(client, "Content-Length: 0")?;
    writeln!(client, "Content-Type: text/html")?;

    Ok(())
}

fn handle_put(client: &mut TcpStream, path: &str, headers: &HashMap<String, String>, body: &String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(format!("{}{}", PUBLIC_DIR, path))?;
    file.write_all(body.as_bytes())?;
    file.flush()?;

    writeln!(client, "HTTP/1.0 200 OK")?;
    writeln!(client, "Content-Length: 0")?;
    writeln!(client, "Content-Type: text/html")?;
    writeln!(client)?;

    Ok(())
}

fn handle_delete(client: &mut TcpStream, path: &str, headers: &HashMap<String, String>, body: &String) -> std::io::Result<()> {
    let path = format!("{}{}", PUBLIC_DIR, path);
    let status = match File::open(&path) {
        Ok(_file) => {
            std::fs::remove_file(&path)?;
            "200 OK"
        },
        Err(_e) => "404 Not Found",
    };

    writeln!(client, "HTTP/1.0 200 OK")?;
    writeln!(client, "Content-Length: 0")?;
    writeln!(client, "Content-Type: text/html")?;
    writeln!(client)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:8080")?;
    let (mut client, _addr) = server.accept()?;
    let mut reader = BufReader::new(&client);

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let (method, path, version) = parse_headline(&buffer);

    println!("Method: {:?}", method);
    println!("Path: {:?}", path);
    println!("Version: {:?}", version);

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

    println!("Headers");
    for (key, value) in headers.iter() {
        println!("Key: {:?}, Value: {:?}", key, value);
    }

    let content_length = if let Some(len) = headers.get("Content-Length") {
        len.parse().unwrap()
    } else {
        0
    };
    let mut body = String::from_utf8(vec![0; content_length]).unwrap();
    println!("{:?}", body.len());
    reader.read_exact(unsafe { body.as_bytes_mut() })?;
    println!("{:?}", body);

    match method {
        "GET" => handle_get(&mut client, path, &headers, &body)?,
        "POST" => handle_post(&mut client, path, &headers, &body)?,
        "PUT" => handle_put(&mut client, path, &headers, &body)?,
        "DELETE" => handle_delete(&mut client, path, &headers, &body)?,
        _ => {},
    }

    client.flush()?;
    client.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
