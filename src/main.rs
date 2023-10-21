use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use http_server_starter_rust::http::HTTPRequest;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                create_response(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn create_response(mut stream: TcpStream) {
    let mut buf = [0; 128];
    stream.read(&mut buf).unwrap();
    let request: HTTPRequest = HTTPRequest::try_from(&buf);

    println!("{:?}", request);

    if request.start_line.path == "/" {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .expect("should write to stream");
        stream.flush().expect("flush stream");
    } else if request.start_line.path.contains("/echo") {
        let (_, r) = request.start_line.path.split_once("/echo/").unwrap();
        let length = r.len();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length:{}\r\n\r\n{}",
            length, r
        );
        println!("{:?}", response);
        stream.write(response.as_bytes()).expect("write to stream");
        stream.flush().expect("flush stream");
    } else {
        stream
            .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .expect("should write to stream");
        stream.flush().expect("flush stream");
    }
}
