use tide::{ Request, Result, Response, StatusCode, prelude::* };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    name: String,
    age: u32,
}

async fn hello(req: Request<()>) -> Result {
    println!("Received request: {:?}", req);
    Ok("Hello, Tide!".into())
}

async fn handle_post(mut req: Request<()>) -> Result {
    let user_data: UserData = req.body_json().await?;

    if user_data.name.is_empty() {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    if user_data.age <= 0 {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    Ok(
        json!({
        "message": "Data received successfully",
        "data": user_data
    }).into()
    )
}

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();

    app.at("/hello").get(hello);
    app.at("/data").post(handle_post);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
