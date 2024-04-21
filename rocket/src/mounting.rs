//======================================================================================
//MOUNTING ROUTES AT ROOT

#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
}

#[get("/about")]
fn about() -> &'static str {
    "About page"
}

#[rocket::main]
async fn main() {
    rocket
        ::build()
        .mount("/", routes![index, about])
        .launch().await
        .expect("Failed to launch Rocket server");
}

//======================================================================================
//MOUNTING ROUTES AT SPECIFIC PATH

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[get("/api")]
fn api() -> &'static str {
    "API endpoint"
}

#[rocket::main]
async fn main() {
    rocket
        ::build()
        .mount("/app", routes![hello, api])
        .launch().await
        .expect("Failed to launch Rocket server");
}

//======================================================================================
//NESTED MOUNTING

#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
}

#[get("/about")]
fn about() -> &'static str {
    "About page"
}

#[get("/contact")]
fn contact() -> &'static str {
    "Contact page"
}

#[rocket::main]
async fn main() {
    rocket
        ::build()
        .mount("/", routes![index])
        .mount("/info", routes![about, contact])
        .launch().await
        .expect("Failed to launch Rocket server");
}
