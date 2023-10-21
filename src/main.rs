use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf = [0; 128];
                _stream.read(&mut buf).unwrap();
                let request = std::str::from_utf8(&buf).unwrap();

                let request: Vec<_> = request.split(" ").collect();
                println!("{:?}", request.clone());
                if request[1] == "/" {
                    _stream
                        .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                        .expect("should write to stream");
                    _stream.flush().expect("flush stream");
                } else {
                    _stream
                        .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                        .expect("should write to stream");
                    _stream.flush().expect("flush stream");
                }
                // _stream
                //     .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                //     .expect("should write to stream");
                // _stream.flush().expect("flush stream");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
