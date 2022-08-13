use warp::Filter;
use serde::{Serialize, Deserialize};

use server::handle_request;


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    list: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    request_type: String,
    source: [i32; 2],
    target: [i32; 2],
    graph: Vec<Vec<char>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: Vec<String>,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /
    let index = warp::path::end().map(|| "Hello, World at root!");

    // GET /api
    let api_route = warp::path("api").map(|| {
        let user = User {
            name: "joe".to_string(),
            age: 30,
            list: vec![1, 2, 3],
        };

        warp::reply::json(&user)
    });

    // POST /api/dijkstra/:name/:age
    let dijkstra_route = warp::path!("api" / "dijkstra")
    .and(warp::body::json())
    .map(|request: Request| {
        let response = Response {
            response: handle_request(request.request_type, request.source, request.target, request.graph),
        };
        warp::reply::json(&response)
    });

    let get_routes = warp::get().and(
        index
        .or(api_route)
    );

    let post_routes = warp::post().and(
        dijkstra_route
    );

    let routes = get_routes.or(post_routes);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
