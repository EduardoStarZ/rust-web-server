use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        // println!("Conection established!")
        handle_conection(stream);
    }
}

fn handle_conection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let contents: String;
    let status_line: &str;

    if buffer.starts_with(get) {
        contents = fs::read_to_string("index.html").unwrap();
        status_line = "HTTP/1.1 200 OK";

} else {
        contents = fs::read_to_string("404.html").unwrap();
        status_line = "HTTP/1.1 404 NOT FOUND";
}

let response: String = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
);

stream.write(response.as_bytes()).unwrap();

        stream.flush().unwrap();
}