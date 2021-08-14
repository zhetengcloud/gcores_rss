use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024 * 4];
    stream.read(&mut buffer).unwrap();

    let contents = buffer
        .iter()
        .skip_while(|&&x| x != b'{')
        .map(|&x| x)
        .collect::<Vec<u8>>();

    let body = String::from_utf8(contents).unwrap_or("body string error".to_string());

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
