use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

pub struct Express {/* TODO */}

impl Express {
    pub fn new() -> Self {
        Express {}
    }
    pub fn get<F>(&mut self, path: &str, callback: F)
    where
        F: FnMut(Request, Response) -> (),
        Self: Sized,
    {
    }

    // TODO: Constraint data type to UNIX port specification
    pub fn listen(&self, port: usize) {
        let address = "0.0.0.0:".to_owned() + &port.to_string();
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut buffer = [0; 1024];
            stream.unwrap().read(&mut buffer).unwrap();
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }
    }
}

pub struct Request {/* TODO */}
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
