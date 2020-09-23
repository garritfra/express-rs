use crate::Mount;
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
    pub params: HashMap<String, String>,
}

impl Request {
    /// takes a request as string and parses all relevant fields
    pub fn from_string(mounts: &mut std::vec::Vec<Mount>, s: String) -> Result<Self, &'static str> {
        let fields = s.split_whitespace().collect::<Vec<_>>();
        let path = parse_path(&fields)?;
        let method = parse_method(&fields)?;

        let params = if let Some(current_mount) = mounts
            .iter_mut()
            .filter(|mount| mount.matches_path(&path))
            .next()
        {
            parse_params(current_mount, &path)
        } else {
            HashMap::new()
        };

        Ok(Request {
            method: method,
            path: path,
            version: parse_version(&fields)?,
            body: parse_body(&s),
            headers: parse_headers(&s)?,
            params: params,
        })
    }
}

fn parse_params(mount: &Mount, path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (i, section) in mount.path.split("/").enumerate() {
        if section.chars().next() == Some(':') {
            map.insert(
                section.chars().skip(1).collect::<String>(),
                path.split("/").nth(i).unwrap().to_string(),
            );
        }
    }

    map
}

fn parse_version(fields: &[&str]) -> Result<String, &'static str> {
    fields
        .get(2)
        .map(|&s| String::from(s))
        .ok_or("Could not parse HTTP version")
}

fn parse_path(fields: &[&str]) -> Result<String, &'static str> {
    fields
        .get(1)
        .map(|&s| String::from(s))
        .ok_or("Could not parse HTTP version")
}

fn parse_method(fields: &[&str]) -> Result<Method, &'static str> {
    match fields.get(0).cloned() {
        Some("GET") => Ok(Method::GET),
        Some("POST") => Ok(Method::POST),
        Some("PUT") => Ok(Method::PUT),
        Some("PATCH") => Ok(Method::PATCH),
        Some("DELETE") => Ok(Method::DELETE),
        // FIXME: This will recognize things as HTTP methods that are not.
        Some(method) => Ok(Method::UNKNOWN(method.to_string())),
        None => Err("Could not parse HTTP method"),
    }
}

/// Parses the body of a request
fn parse_body(s: &str) -> Option<String> {
    // RFC 7230 Section 3: Body begins after two CRLF (\r\n) sequences.
    // See: https://tools.ietf.org/html/rfc7230#section-3
    let text = s.split("\r\n\r\n").skip(1).collect::<String>();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn parse_headers(s: &str) -> Result<HashMap<String, String>, &'static str> {
    // RFC 7230 Section 3: Header section (start-line) ends, when two CRLF (\r\n) sequences are encountered.
    // See: https://tools.ietf.org/html/rfc7230#section-3
    let raw_header_section = s.split("\r\n\r\n").next().unwrap_or_default();

    // RFC 7230 Section 3.2: Each header is separated by one CRLF.
    // See: https://tools.ietf.org/html/rfc7230#section-3.2
    let raw_headers = raw_header_section.split("\r\n").skip(1).collect::<Vec<_>>();
    let mut map = HashMap::new();

    for header in raw_headers {
        let sections = header.split(':').collect::<Vec<_>>();
        let field_name = sections.get(0);
        let field_value = sections.get(1);

        // RFC 7230 Section 3.2.4: Empty header names or fields render the request invalid.
        // See: https://tools.ietf.org/html/rfc7230#section-3.2.4
        field_name
            .and(field_value)
            .ok_or("Error while parsing request headers")?;

        if let Some(field_name) = field_name {
            // RFC 7230 Section 3.2.4: No whitespace is allowed between the header field-name and colon.
            // See: https://tools.ietf.org/html/rfc7230#section-3.2.4
            if field_name
                .chars()
                .last()
                .filter(|c| c.is_whitespace())
                .is_some()
            {
                return Err("No whitespace is allowed between the header field-name and colon");
            }

            if let Some(field_value) = field_value {
                map.insert(
                    field_name.split_whitespace().collect(),
                    field_value.split_whitespace().collect(),
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
    pub(crate) status: u16,
}

impl Default for Response {
    fn default() -> Self {
        Response::new()
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            stream: String::new(),
            headers: Vec::new(),
            status: 200,
        }
    }

    /// Writes plain text to the response buffer
    pub fn send(&mut self, s: String) {
        self.stream.push_str(&s);
    }

    /// Change the status code of a response
    ///
    /// # Examples
    ///
    /// ```
    /// use express_rs::Express;
    ///
    /// let mut app = Express::new();
    ///
    /// app.get("/", |_, res| {
    ///     res.status(301).send("This route has a custom status code".to_string())
    /// });
    /// ```
    pub fn status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }
}
