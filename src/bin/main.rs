use std::time::Duration;
use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use server::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let _pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        thread::spawn(|| {
            handle_conection(stream);
        });
    }
}

fn handle_conection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let sleep_test: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";
    
    let contents: String;
    let status_line: &str;

    if buffer.starts_with(get) {
        contents = fs::read_to_string("index.html").unwrap();
        status_line = "HTTP/1.1 200 OK";

} else if buffer.starts_with(sleep_test){
    thread::sleep(Duration::from_secs(5));

    contents = fs::read_to_string("sleep.html").unwrap();
    status_line = "HTTP/1.1 200 OK";
} 
else {
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