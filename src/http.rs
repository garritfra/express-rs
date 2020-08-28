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
    pub body: Option<String>,
}

impl Request {
    /// takes an request as string and parses all relevant fields
    pub fn from_string(b: String) -> Result<Self, &'static str> {
        let result = std::panic::catch_unwind(|| {
            let fields: Vec<&str> = b.split_whitespace().collect();
            let body: Option<String> = b.split("\r\n\r\n").nth(1).map(|s| s.to_string());

            let method = match fields.get(0).unwrap() {
                &"GET" => Method::GET,
                &"POST" => Method::POST,
                &"PUT" => Method::PUT,
                &"PATCH" => Method::PATCH,
                &"DELETE" => Method::DELETE,
                method => Method::UNKNOWN(method.to_string()),
            };

            let path = fields.get(1).unwrap().to_string();
            let version = fields.get(2).unwrap().to_string();

            Request {
                method,
                path,
                version,
                body,
            }
        });

        // FIXME: This could be prettier
        if result.is_err() {
            Err("Could not handle request")
        } else {
            Ok(result.unwrap())
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
