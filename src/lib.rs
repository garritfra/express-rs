use std::fmt::Debug;
use std::io::Read;
use std::net::TcpListener;

pub struct Express {
    mounts: Vec<Mount>,
}

impl Express {
    pub fn new() -> Self {
        Express { mounts: Vec::new() }
    }
    pub fn get<F: 'static>(&mut self, path: &str, callback: F) -> &mut Self
    where
        F: FnMut(Request, Response) -> (),
        Self: Sized,
    {
        let mount = Mount {
            method: Method::GET,
            path: path.to_string(),
            callback: Box::new(callback),
        };
        self.mounts.push(mount);

        self
    }

    /// Port numbers can range from 1-65535, therefore a u16 is used here
    ///
    /// # Panics
    /// Panics, if a port is not between 1-65535
    pub fn listen(&self, port: u16) {
        if port == 0 {
            panic!("Port must be between 1-65535")
        }

        let address = "0.0.0.0:".to_string() + &port.to_string();
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut buffer = [0; 1024];
            stream.unwrap().read(&mut buffer).unwrap();
            let request = Request::from_string(String::from_utf8_lossy(&buffer[..]).to_string());

            for mount in &self.mounts {
                if mount.path == request.path && mount.method == request.method {}
            }

            println!("{:?}", request);
        }
    }
}

/// Mounts are essentially REST routes. They define a method and a path
pub struct Mount {
    pub method: Method,
    pub path: String,
    callback: Box<dyn FnMut(Request, Response) -> ()>,
}

impl Debug for Mount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Mount")
            .field("method", &self.method)
            .field("path", &self.path)
            .finish()
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    UNKNOWN,
    // ...
}

#[derive(Debug, PartialEq)]
pub struct Request {
    method: Method,
    path: String,
    version: String,
}

/*
Request: GET / HTTP/1.1
Host: localhost:8080
Connection: keep-alive
Cache-Control: max-age=0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.135 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng
Sec-Fetch-Site: cross-site
Sec-Fetch-Mode: navigate
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en-CA,en;q=0.9,de-DE;q=0.8,de;q=0.7,en-GB;q=0.6,en-US;q=0.5,la;q=0.4
Cookie: _ga=GA1.1.783631210.1557768372
*/

impl Request {
    pub fn from_string(b: String) -> Self {
        let fields: Vec<&str> = b.split_whitespace().collect();
        Request {
            method: match fields.get(0).unwrap() {
                &"GET" => Method::GET,
                _ => Method::UNKNOWN,
            },
            path: fields.get(1).unwrap().to_string(),
            version: fields.get(2).unwrap().to_string(),
        }
    }
}

pub struct Response {/* TODO */}

impl Response {
    pub fn send(&mut self, s: String) {
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
