#[derive(Debug)]
pub struct HTTPRequest {
    pub start_line: StartLine,
    pub headers: Option<Vec<String>>,
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

impl HTTPRequest {
    pub fn try_from(buffer: &[u8]) -> Self {
        let request = std::str::from_utf8(&buffer).unwrap();

        let (start_line_string, _rest) = request.split_once("\r\n").unwrap();
        let res: Vec<_> = start_line_string.split(" ").collect();
        //println!("start : {start_line_string} , rest: {_rest} REST FINISHED");
        let method = match res[0] {
            "GET" => HTTPMethod::GET,
            _ => HTTPMethod::GET,
        };
        let start = StartLine {
            method: method,
            path: res[1].to_string(),
            version: res[2].to_string(),
        };

        HTTPRequest {
            start_line: start,
            headers: None,
            body: None,
        }
    }
}
