//use itertools::Itertools;

#[derive(Debug)]
pub struct HTTPRequest {
    pub start_line: StartLine,
    pub headers: Option<Vec<Header>>,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HTTPMethod {
    GET,
    POST,
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
        println!("remaidner: {remainder}");
        let start_line_vec: Vec<_> = start_line_string.split(" ").collect();

        let method = match start_line_vec[0] {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            _ => HTTPMethod::GET,
        };
        let start = StartLine {
            method,
            path: start_line_vec[1].to_string(),
            version: start_line_vec[2].to_string(),
        };

        let opt = remainder.split_once("\r\n\r\n");
        let (headers, body) = match opt {
            Some((headers, body)) => {
                let headers = headers
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
                let mut body_re = None;
                if let Some((body, _)) = body.split_once("\0") {
                    if body.is_empty() {
                        body_re = None;
                    } else {
                        body_re = Some(body.to_string())
                    }
                }
                (Some(headers), body_re)
            }
            None => (None, None),
        };

        HTTPRequest {
            start_line: start,
            headers: headers,
            body: body,
        }
    }
}
