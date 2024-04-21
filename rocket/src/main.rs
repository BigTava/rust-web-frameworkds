#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::config::{ Config, Environment };

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html("<h1>Hello, Rocket!</h1>")
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[rocket::main]
async fn main() {
    let config = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(8000)
        .finalize()
        .unwrap();

    rocket
        ::custom(config)
        .mount("/", routes![index, hello])
        .launch().await
        .expect("Failed to launch Rocket server");
}
