use std::borrow::Cow;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "1337";
    let end_point: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening at port {} 🚀.", PORT);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    handle_request(request, &mut stream);

    stream.flush().unwrap();
}

fn handle_request(request: Cow<'_, str>, stream: &mut TcpStream) {
    if request.starts_with("GET /ping HTTP/1.1") {
        let response = "HTTP/1.1 200 OK\r\n\r\n🏓 pong!\n";
        stream.write(response.as_bytes()).unwrap();
        return;
    }

    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned();
    stream.write(response.as_bytes()).unwrap();
    return;
}
