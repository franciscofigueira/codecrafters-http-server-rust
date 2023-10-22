use anyhow::Result;
use http_server_starter_rust::http::{HTTPMethod, HTTPRequest};
use std::env;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    stream.read(&mut buf).await?;
    let request: HTTPRequest = HTTPRequest::try_from(&buf);

    println!("request parsed: {:?}", request);
    let mut reponse_string = String::new();

    let response = if request.start_line.method == HTTPMethod::POST {
        let args: Vec<_> = env::args().collect();
        let directory = &args[2];
        let (_, file_name) = request.start_line.path.split_once("/files/").unwrap();
        let file_path = directory.to_owned() + file_name;

        tokio::fs::write(file_path, request.body.unwrap()).await?;

        "HTTP/1.1 201 OK\r\n\r\n".as_bytes()
    } else if request.start_line.path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n".as_bytes()
    } else if request.start_line.path.contains("/echo") {
        let (_, r) = request.start_line.path.split_once("/echo/").unwrap();
        let length = r.len();
        reponse_string = format!(
            "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length:{}\r\n\r\n{}",
            length, r
        );
        reponse_string.as_bytes()
    } else if request.start_line.path.contains("/user-agent") {
        if let Some(headers) = request.headers.as_ref() {
            for header in headers {
                if header.key == "User-Agent" {
                    reponse_string = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length:{}\r\n\r\n{}",
                        header.value.len(),
                        header.value
                    );
                    break;
                }
            }
        }
        reponse_string.as_bytes()
    } else if request.start_line.path.contains("/files/") {
        let args: Vec<_> = env::args().collect();
        let directory = &args[2];
        let (_, file_name) = request.start_line.path.split_once("/files/").unwrap();
        let file_path = directory.to_owned() + file_name;
        let res: &[u8];
        if let Ok(file_contents) = tokio::fs::read_to_string(file_path).await {
            reponse_string =
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", 
                file_contents.len(), file_contents);
            res = reponse_string.as_bytes();
        } else {
            res = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes();
        };

        res
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()
    };

    let _written = stream.write_all(response).await?;
    stream.flush().await?;

    Ok(())
}
