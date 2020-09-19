use std::collections::hash_map::HashMap;

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
    pub headers: HashMap<String, String>,
}

impl Request {
    /// takes an request as string and parses all relevant fields
    pub fn from_string(b: String) -> Result<Self, &'static str> {
        let result = std::panic::catch_unwind(|| {
            let body: String = parse_body(&b);
            Request {
                method: parse_method(&b),
                path: parse_path(&b),
                version: parse_version(&b),
                body: if body.is_empty() { None } else { Some(body) },
                headers: parse_headers(&b).unwrap(),
            }
        });

        // FIXME: This could be prettier
        if result.is_err() {
            return Err("Could not handle request");
        } else {
            return Ok(result.unwrap());
        }
    }
}

fn parse_version(s: &String) -> String {
    let fields: Vec<&str> = s.split_whitespace().collect();
    fields.get(2).unwrap().to_string()
}

fn parse_path(s: &String) -> String {
    let fields: Vec<&str> = s.split_whitespace().collect();
    fields.get(1).unwrap().to_string()
}

fn parse_method(s: &String) -> Method {
    let fields: Vec<&str> = s.split_whitespace().collect();
    match fields.get(0).unwrap() {
        &"GET" => Method::GET,
        &"POST" => Method::POST,
        &"PUT" => Method::PUT,
        &"PATCH" => Method::PATCH,
        &"DELETE" => Method::DELETE,
        method => Method::UNKNOWN(method.to_string()),
    }
}

/// Parses the body of a request
fn parse_body(s: &String) -> String {
    // RFC 7230 Section 3: Body begins after two CRLF (\r\n) sequences.
    // See: https://tools.ietf.org/html/rfc7230#section-3
    s.split("\r\n\r\n").skip(1).collect::<String>()
}

fn parse_headers(s: &String) -> Result<HashMap<String, String>, &str> {
    // RFC 7230 Section 3: Header section (start-line) ends, when two CRLF (\r\n) sequences are encountered.
    // See: https://tools.ietf.org/html/rfc7230#section-3
    let raw_header_section: &str = s.split("\r\n\r\n").nth(0).unwrap_or("");

    // RFC 7230 Section 3.2: Each header is separated by one CRLF.
    // See: https://tools.ietf.org/html/rfc7230#section-3.2
    let raw_headers: Vec<&str> = raw_header_section.split("\r\n").skip(1).collect();
    let mut map = HashMap::new();

    for header in raw_headers {
        let sections: Vec<&str> = header.split(":").collect();
        let field_name = sections.get(0);
        let field_value = sections.get(1);

        // RFC 7230 Section 3.2.4: Empty header names or fields render the request invalid.
        // See: https://tools.ietf.org/html/rfc7230#section-3.2.4
        if field_name.is_none() || field_value.is_none() {
            return Err("Error while parsing request headers");
        }

        if let Some(field_name) = field_name {
            // RFC 7230 Section 3.2.4: No whitespace is allowed between the header field-name and colon.
            // See: https://tools.ietf.org/html/rfc7230#section-3.2.4
            if let Some(char_after_fieldname) = field_name.chars().last() {
                if char_after_fieldname.is_whitespace() {
                    return Err("No whitespace is allowed between the header field-name and colon");
                }
            }

            if let Some(field_value) = field_value {
                map.insert(
                    field_name.to_string().split_whitespace().collect(),
                    field_value.to_string().split_whitespace().collect(),
                );
            }
        }
    }

    Ok(map)
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
