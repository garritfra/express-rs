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
    }
}
