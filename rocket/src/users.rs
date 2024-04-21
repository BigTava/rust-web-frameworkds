#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::State;
use rocket_contrib::json::{ Json, JsonValue };
use std::sync::Mutex;

// Define a structure to represent a User
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
    email: String,
}

// Create a shared state to store users (using Mutex for thread safety)
type UsersDB = Mutex<Vec<User>>;

// Add a user to the database
#[post("/users", format = "json", data = "<user>")]
fn add_user(user: Json<User>, users_db: State<UsersDB>) -> JsonValue {
    let mut users = users_db.lock().unwrap();
    users.push(user.into_inner());
    json!({ "status": "User added successfully" })
}

// Get all users from the database
#[get("/users")]
fn get_users(users_db: State<UsersDB>) -> Json<Vec<User>> {
    let users = users_db.lock().unwrap();
    Json(users.clone())
}

// Update a user in the database by ID
#[put("/users/<id>", format = "json", data = "<user>")]
fn update_user(id: usize, user: Json<User>, users_db: State<UsersDB>) -> JsonValue {
    let mut users = users_db.lock().unwrap();
    for u in users.iter_mut() {
        if u.id == id {
            u.name = user.name.clone();
            u.email = user.email.clone();
            return json!({ "status": "User updated successfully" });
        }
    }
    json!({ "status": "User not found" })
}

// Delete a user from the database by ID
#[delete("/users/<id>")]
fn delete_user(id: usize, users_db: State<UsersDB>) -> JsonValue {
    let mut users = users_db.lock().unwrap();
    let initial_len = users.len();
    users.retain(|u| u.id != id);
    if users.len() < initial_len {
        json!({ "status": "User deleted successfully" })
    } else {
        json!({ "status": "User not found" })
    }
}

fn rocket() -> rocket::Rocket {
    rocket
        ::ignite()
        .mount("/", routes![add_user, get_users, update_user, delete_user])
        .manage(Mutex::new(Vec::<User>::new())) // Initialize empty user database
}

fn main() {
    rocket().launch();
}
