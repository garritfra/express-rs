/// Common HTTP Methods
///
/// If a method is needed, which is not specified here,
/// `Method::UNKNOWN(String)` can be used
#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    UNKNOWN(String),
}

/// Represents a HTTP request
#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
}

impl Request {
    /// takes an request as string and parses all relevant fields
    pub fn from_string(b: String) -> Self {
        let fields: Vec<&str> = b.split_whitespace().collect();
        Request {
            method: match fields.get(0).unwrap() {
                &"GET" => Method::GET,
                &"POST" => Method::POST,
                &"PUT" => Method::PUT,
                &"PATCH" => Method::PATCH,
                &"DELETE" => Method::DELETE,
                method => Method::UNKNOWN(method.to_string()),
            },
            path: fields.get(1).unwrap().to_string(),
            version: fields.get(2).unwrap().to_string(),
        }
    }
}

/// Represents a HTTP response
#[derive(Debug, PartialEq)]
pub struct Response {
    pub stream: String,
    pub headers: Vec<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            stream: String::new(),
            headers: Vec::new(),
        }
    }

    /// Writes plain text to the response buffer
    pub fn send(&mut self, s: String) {
        self.stream.push_str(&s);
    }
}
