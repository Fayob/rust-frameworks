use tide::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    name: String,
    age: u32,
}


async fn hello_world(req: tide::Request<()>) -> tide::Result {
    println!("Received request: {:?}", req);

    Ok("Hello, World!".into())
}

async fn handle_post(req: tide::Request<()>) -> tide::Result {
    println!("Received POST request: {:?}", req);

    let data: Data = req.body_json().await?;
    println!("Parsed data: {:?}", data);

    if data.name.is_empty() || data.age == 0 {
        return Ok(tide::Response::new(tide::StatusCode::BadRequest).body_string("Invalid data provided"));
    }

    Ok(json!({
        "Received name: {}, age: {}",
        data.name, data.age
    })).into()
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/tide/hello_world").get(hello_world);
    app.at("/data").post(handle_post);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
