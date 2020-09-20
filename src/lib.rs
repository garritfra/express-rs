pub mod http;

use http::{Method, Request, Response};
use std::fmt::Debug;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

pub struct Express {
    mounts: Vec<Mount>,
}

/// This macro saves a ton of work when adding new HTTP methods.
/// It generates struct methods that specify the HTTP method, path, and callback.define_method!
///
/// # Examples
/// ```
/// define_method!(get, Method::GET);
///
/// // exposes
///
/// app.get("/", |req, res| res.send("Hello"))
///
/// ```
macro_rules! define_method {
    ($func_name:ident, $method:expr) => {
        pub fn $func_name<F: 'static>(&mut self, path: &str, callback: F) -> &mut Self
        where
            F: FnMut(&Request, &mut Response),
            Self: Sized,
        {
            let mount = Mount {
                method: $method,
                path: path.to_string(),
                callback: Box::new(callback),
            };
            self.mounts.push(mount);
            self
        }
    };
}

/// Main application object
///
/// Provides ways to mount path and method combinations
/// and assign functions to them.
impl Express {
    pub fn new() -> Self {
        Express { mounts: Vec::new() }
    }

    define_method!(get, Method::GET);
    define_method!(post, Method::POST);
    define_method!(put, Method::PUT);
    define_method!(delete, Method::DELETE);
    define_method!(patch, Method::PATCH);

    /// Port numbers can range from 1-65535, therefore a u16 is used here
    ///
    /// # Panics
    /// Panics, if:
    /// - a port is not between 1-65535 or
    /// - address on host is already in use
    pub fn listen(&mut self, port: u16) {
        if port == 0 {
            panic!("Port must be between 1-65535")
        }

        let address = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(address)
            .unwrap_or_else(|_| panic!("Could not bind to port {}", port));

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                if let Err(e) = stream.read(&mut buffer) {
                    println!("Could not read to stream: {}", e)
                }
                let request =
                    Request::from_string(String::from_utf8_lossy(&buffer[..]).to_string());

                let mut response = Response::new();
                if let Ok(request) = request {
                    self.mounts
                        .iter_mut()
                        .filter(|mount| mount.matches(&request))
                        .for_each(|mount| (mount.callback)(&request, &mut response));

                    let response_text = format!("HTTP/1.1 {} OK\r\n\r\n", response.status);
                    if let Err(e) = stream.write(response_text.as_bytes()) {
                        println!("Could not write to response stream: {}", e)
                    }
                } else {
                    if let Err(e) = stream.write(b"HTTP/1.1 400 Bad Request\r\n\r\n") {
                        println!("Could not write to response stream: {}", e)
                    }
                    println!("Request could not be handled");
                }

                if let Err(e) = stream.write(response.stream.as_bytes()) {
                    println!("Could not write to response stream: {}", e)
                }

                if let Err(e) = stream.flush() {
                    println!("Could not flush response stream: {}", e)
                }
            }
        }
    }
}

impl Default for Express {
    fn default() -> Self {
        Express::new()
    }
}

/// Represents a path with a method.
pub struct Mount {
    pub method: Method,
    pub path: String,
    pub callback: Box<dyn FnMut(&Request, &mut Response)>,
}

impl Debug for Mount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Mount")
            .field("method", &self.method)
            .field("path", &self.path)
            .finish()
    }
}

impl Mount {
    fn matches(&self, other: &Request) -> bool {
        self.method == other.method && self.path == other.path
    }
}
