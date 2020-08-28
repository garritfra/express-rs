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
            F: FnMut(&Request, &mut Response) -> (),
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
    /// Panics, if a port is not between 1-65535
    pub fn listen(&mut self, port: u16) {
        if port == 0 {
            panic!("Port must be between 1-65535")
        }

        let address = "0.0.0.0:".to_string() + &port.to_string();
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                let request =
                    Request::from_string(String::from_utf8_lossy(&buffer[..]).to_string());
                let mut response = Response::new();

                for mount in &mut self.mounts {
                    if mount.path == request.path && mount.method == request.method {
                        (mount.callback)(&request, &mut response);
                    }
                }
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                stream.write(response.stream.as_bytes()).unwrap();
                stream.flush().unwrap();

                println!("Request: {:?}", request);
                println!("Response: {:?}", response);
            }
        }
    }
}

/// Represents a path with a method.
pub struct Mount {
    pub method: Method,
    pub path: String,
    pub callback: Box<dyn FnMut(&Request, &mut Response) -> ()>,
}

impl Debug for Mount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Mount")
            .field("method", &self.method)
            .field("path", &self.path)
            .finish()
    }
}
