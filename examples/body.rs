use express_rs::Express;

/// This example demonstrates how to access the body of a request.
/// Test it by POSTing a request with any body to port 8080
fn main() {
    let mut app = Express::new();
    app.post("/", |req, res| {
        if let Some(body) = &req.body {
            res.send(body.to_string());
        } else {
            res.send("Nobody here...".to_string());
        }
    });

    let port = 8080;
    println!("Starting server on port {}", port);
    app.listen(port);
}
