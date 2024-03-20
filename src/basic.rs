use warp::Filter;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct Data {
    data: Vec<String>,
}

#[tokio::main]
async fn main() {
    let run_route = warp::path("run")
        .and(warp::post())
        .and(warp::body::json())
        .map(|data: Data| warp::reply::json(&data));

    warp::serve(run_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

