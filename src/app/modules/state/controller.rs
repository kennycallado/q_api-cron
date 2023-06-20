
pub fn routes() -> Vec<rocket::Route> {
    routes![
        hello,
    ]
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello from state"
}
