# express-rs

This crate emulates the behavior of the Express.js framework for Rust.

## State of the Project

Very early WIP!

## Example

See [./examples](./examples) for more examples

```rust
use express_rs::Express;

fn main() {
    let mut app = Express::new();

    app.get("/", |_, res| res.send("Hello World!".to_string()));

    app.listen(8080);
}
```
