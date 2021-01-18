#[get("/")]
fn desc() -> &'static str {
    "You can play games"
}


pub fn main() {
    rocket::ignite().mount(
        "/game",
        routes![desc]
    ).launch();
}
