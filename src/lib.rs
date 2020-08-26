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
        todo!()
    }

    // TODO: Constraint data type to UNIX port specification
    pub fn listen(&self, port: usize) {}
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
