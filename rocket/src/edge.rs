//======================================================================================
//Using Experimental Syntax:

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
}

#[rocket::main]
async fn main() {
    rocket
        ::ignite()
        .mount("/", routes![index])
        .launch().await
        .expect("Failed to launch Rocket server");
}

//======================================================================================
//Using Edge Features for Rocket Configuration:

#![feature(decl_macro)];

#[macro_use]
extern crate rocket;

use rocket::config::{ Config, Environment };

#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
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
        .mount("/", routes![index])
        .launch().await
        .expect("Failed to launch Rocket server");
}

//======================================================================================
//Using Experimental Features in Route Definitions:

#![feature(proc_macro_hygiene, decl_macro)];

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[rocket::main]
async fn main() {
    rocket
        ::ignite()
        .mount("/", routes![index, hello])
        .launch().await
        .expect("Failed to launch Rocket server");
}
