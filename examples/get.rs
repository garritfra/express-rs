use express_rs::Express;

fn main() {
    let mut app = Express::new();
    app.get("/", |_, mut res| res.send("Hello World!".to_string()));

    app.listen(8080);
}
