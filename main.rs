// use log::{info, arn, errors};
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  let port = "127.0.0.1:8080";
  let listener = TcpListener::bind(port).unwrap();

  for stream in listener.incoming() {
    println!("Server running on port {}", port);
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  let html = fs::read_to_string("./index.html").unwrap();
  println!("Sending the index.html page via rust");

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    html.len(),
    html
  );
  
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
