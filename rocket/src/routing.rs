//BASIC
#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
}

//======================================================================================
//DYNAMIC

#[get("/hello/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

//======================================================================================
//QUERY PARAMETERS

#[get("/search?<query>")]
fn search(query: String) -> String {
    format!("Searching for: {}", query)
}

//======================================================================================
//MULTIPLE ROUTES
#[get("/")]
fn index() -> &'static str {
    "Welcome to the home page!"
}

#[get("/about")]
fn about() -> &'static str {
    "About page"
}

//======================================================================================
//ROUTES WITH GUARDS

#[get("/admin")]
fn admin_panel() -> &'static str {
    "Admin panel"
}

#[get("/admin", rank = 2)]
fn unauthorized_admin() -> &'static str {
    "Unauthorized access to admin panel"
}

#[catch(404)]
fn not_found() -> &'static str {
    "Page not found"
}
