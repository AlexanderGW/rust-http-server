use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::time::Duration;

use rust_http_server::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8192").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        pool.execute( || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let sleep: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";

    let (status_code, status_label, filename) =
    if buffer.starts_with(get) {
        (200, "OK", "./public/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (200, "OK", "./public/index.html")
    } else {
            (404, "Not Found", "./404.html")
        };

    let contents: String = fs::read_to_string(filename).unwrap();

    let response: String = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        status_label,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    // stream.write(content.as_bytes()).unwrap();
    stream.flush().unwrap();

}
