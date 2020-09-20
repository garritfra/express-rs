# express-rs

This crate emulates the behavior of the Express.js framework for Rust.

https://crates.io/crates/express-rs

## State of the Project

- [x] Simple GET/POST/PUT/DELETE requests
- [x] Body parsing
- [x] Status codes
- [x] Header parsing
- [ ] proper HTML rendering
- [ ] Dynamic query and route params
- [ ] Multithreading

## Example

See `examples` directory for more examples

```rust
use express_rs::Express;

fn main() {
    let mut app = Express::new();

    app.get("/", |_, res| res.send("Hello World!".to_string()));

    app.listen(8080);
}
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
