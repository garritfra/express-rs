#[cfg(test)]
mod tests {
    use express_rs::*;

    #[test]
    fn request_from_string_valid_no_body() {
        let string = "GET / HTTP/1.1".to_string();

        let request = http::Request::from_string(string).unwrap();

        assert_eq!(request.method, http::Method::GET);
        assert_eq!(request.path, "/");
        assert_eq!(request.version, "HTTP/1.1".to_string());
        assert_eq!(request.body, None);
    }

    #[test]
    fn request_from_string_valid_with_body() {
        let string = "GET / HTTP/1.1\r\n\r\nthis is the body".to_string();

        let request = http::Request::from_string(string).unwrap();

        assert_eq!(request.method, http::Method::GET);
        assert_eq!(request.path, "/");
        assert_eq!(request.version, "HTTP/1.1".to_string());
        assert_eq!(request.body, Some("this is the body".to_string()));
    }

    #[test]
    fn request_from_string_invalid() {
        let string = "GET".to_string();
        let request = http::Request::from_string(string);
        assert_eq!(request.is_err(), true);
    }

    #[test]
    fn request_from_string_headers_valid() {
        let string =
            "GET / HTTP/1.1\r\nContent-Type: text/plain\r\n\r\nthis is the body".to_string();
        let request = http::Request::from_string(string).unwrap();

        let header = request.headers.get(&"Content-Type".to_string());
        assert_eq!(header, Some(&"text/plain".to_string()));
    }

    /// The space after the colon and after the end of line are optional white space (OWS)
    /// and should therefore be parsed as such.
    /// See: https://tools.ietf.org/html/rfc7230#section-3.2
    #[test]
    fn request_from_string_headers_weird_spacing() {
        let string =
            "GET / HTTP/1.1\r\nContent-Type:text/plain \r\n\r\nthis is the body".to_string();
        let request = http::Request::from_string(string).unwrap();

        let header = request.headers.get(&"Content-Type".to_string());
        assert_eq!(header, Some(&"text/plain".to_string()));
    }

    /// Missing colon should render the request invalid
    #[test]
    fn request_from_string_headers_invalid() {
        let string =
            "GET / HTTP/1.1\r\nContent-Type text/plain \r\n\r\nthis is the body".to_string();
        let request = http::Request::from_string(string);

        assert_eq!(request.is_err(), true);
    }

    /// Space between field-name and colon should render request invalid
    #[test]
    fn request_from_string_headers_space_after_fieldname() {
        let string =
            "GET / HTTP/1.1\r\nContent-Type : text/plain \r\n\r\nthis is the body".to_string();
        let request = http::Request::from_string(string);

        assert_eq!(request.is_err(), true);
    }
}
