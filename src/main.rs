use std::fs::read_to_string;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    let stream = TcpListener::bind("127.0.0.1:3000").unwrap();
    for s in stream.incoming() {
        let stream = s.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let status_line = String::from("HTTP/1.1 200 OK");
    let contents = read_to_string("public/index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    println!("Request handling");
    stream.write_all(response.as_bytes()).unwrap();
}