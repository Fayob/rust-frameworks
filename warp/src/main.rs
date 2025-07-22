use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{ Arc, Mutex };

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: Option<u64>,
    name: String,
}

#[tokio::main]
async fn main() {
    // let hello = warp::path("hello")
    //     .and_then(|| {
    //         async {
    //             Ok::<_, warp::Rejection>(warp::reply::html("Hello, World!"))
    //         }
    //     });

    let items: Arc<Mutex<Vec<Item>>> = Arc::new(Mutex::new(Vec::new()));

    let create_items = warp::path("items" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .map({
            let items = items.clone();
            move |new_item: Item| {
                let mut items = items.lock().unwrap();
                let id = items.len() as u64 + 1;
                let item = Item { id: Some(id), name: new_item.name };
                items.push(item.clone());
                warp::reply::json(&item)
            }
        });
    
    let get_all_items = warp::path("items" / "all")
        .and(warp::get())
        .map({
            let items = items.clone();
            move || {
                let items = items.lock().unwrap();
                warp::reply::json(&items.clone)
            }
        });

    // combine filters into the main API
    let api = create_items.or(get_all_items);

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
