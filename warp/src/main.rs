use warp::Filter;
use serde::{ Serialize, Deserialize };
use std::{ collections::HashMap, sync::{ Arc, Mutex } };
use tokio::sync::Mutex as AsyncMutex;

// Define a simple data structure to represent our resource
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    id: Option<u64>,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Employee {
    id: u32,
    name: String,
}

type EmployeeMap = Arc<AsyncMutex<HashMap<u32, String>>>;

async fn read_employee_name(id: u32, employees: EmployeeMap) -> Option<String> {
    let guard = employees.lock().await;
    guard.get(&id).cloned()
}

#[tokio::main]
async fn main() {
    // Initialize the vector to store items
    let items: Arc<Mutex<Vec<Item>>> = Arc::new(Mutex::new(Vec::new()));

    // Define a filter to create an item
    let create_item = warp
        ::path("items/create")
        .and(warp::body::json())
        .map({
            let items = items.clone();
            move |new_item: Item| {
                let mut items = items.lock().unwrap();
                let id = (items.len() as u64) + 1;
                let item = Item { id: Some(id), name: new_item.name };
                items.push(item.clone());
                warp::reply::json(&item)
            }
        });

    // Define a filter to get all items
    let get_all_items = warp::path("items/all").map({
        let items = items.clone();
        move || {
            let items = items.lock().unwrap();
            warp::reply::json(&items.clone())
        }
    });

    let employees: EmployeeMap = Arc::new(AsyncMutex::new(HashMap::new()));
    let get_employee_name = warp
        ::path!("employees" / u32)
        .and(warp::any().map(move || employees.clone()))
        .and_then(|id, employees: EmployeeMap| async move {
            match read_employee_name(id, employees).await {
                Some(name) => Ok::<_, warp::Rejection>(warp::reply::json(&Employee { id, name })),
                None => Ok(warp::reply::json(&format!("Employee with ID {} not found", id))),
            }
        });

    // Combine filters into the main API
    let api = warp::post().and(create_item.or(get_all_items).or(get_employee_name));

    // Run the server
    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
