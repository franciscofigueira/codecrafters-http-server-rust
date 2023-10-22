//use itertools::Itertools;

#[derive(Debug)]
pub struct HTTPRequest {
    pub start_line: StartLine,
    pub headers: Option<Vec<Header>>,
    pub body: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
}

#[derive(Debug)]
pub struct StartLine {
    pub method: HTTPMethod,
    pub path: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl HTTPRequest {
    pub fn try_from(buffer: &[u8]) -> Self {
        let request = std::str::from_utf8(&buffer).unwrap();

        let (start_line_string, remainder) = request.split_once("\r\n").unwrap();
        let (headers, _body) = remainder.split_once("\r\n\r\n").unwrap();
        let start_line_vec: Vec<_> = start_line_string.split(" ").collect();

        let headers: Vec<Header> = headers
            .split("\r\n")
            .map(|header| match header.split_once(": ") {
                Some((key, value)) => Header {
                    key: key.to_string(),
                    value: value.to_string(),
                },
                None => Header {
                    key: "".to_string(),
                    value: "".to_string(),
                },
            })
            .collect();

        let method = match start_line_vec[0] {
            "GET" => HTTPMethod::GET,
            _ => HTTPMethod::GET,
        };
        let start = StartLine {
            method,
            path: start_line_vec[1].to_string(),
            version: start_line_vec[2].to_string(),
        };

        HTTPRequest {
            start_line: start,
            headers: Some(headers),
            body: None,
        }
    }
}
