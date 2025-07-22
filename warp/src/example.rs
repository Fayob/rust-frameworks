// This code should be in the main file but I don't want to have a conflict with the previous code that's why I put it here

use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Employee {
    id: u32,
    name: String,
}

type EmployeeMap = Arc<AsyncMutex<HashMap<u32, String>>>;

async fn fetch_employee_name(
    id: u32,
    employees: EmployeeMap,
) -> Option<String> {
    let employees = employees.lock().await;
    employees.get(&id).cloned()
}

async fn get_employee_name(id: u32, employees: EmploymentMap) -> Result<impl Reply, warp::Rejection> {
    match fetch_employee_name(id, employees).await {
        Some(name) => Ok(warp::reply::json(&Employee { id, name })),
        None => Ok(warp::reply::json(&format!("Employee with ID {} not found", id))),
    }
}

#[tokio::main]
async fn main() {
    let employees: EmployeeMap = Arc::new(AsyncMutex::new(HashMap::new()));

    {
        let mut employees = employees.lock().await;
        employees.insert(1, "Alice".to_string());
        employees.insert(2, "Bob".to_string());
    }

    let get_employee_name_route = warp::path!("employee" / u32)
        .and(warp::get())
        .and(warp::any().map(move || employees.clone()))
        .and_then(get_employee_name);

    warp::serve(get_employee_name_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
