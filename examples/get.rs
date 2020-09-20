use express_rs::Express;

fn main() {
    let mut app = Express::new();
    app.get("/", |_, res| res.send("Hello World!".to_string()));
    app.get("/hello", |_, res| {
        res.send("<h1>Hi from /hello!</h1>".to_string())
    });

    app.get("/redirect", |_, res| {
        res.status(301)
            .send("This route has a redirect status code!".to_string())
    });

    app.listen(8080);
}
