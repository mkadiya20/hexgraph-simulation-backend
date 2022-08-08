use warp::Filter;
use serde::{Serialize, Deserialize};

// use server::test_lib;


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    list: Vec<i32>,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /
    let index = warp::path::end().map(|| "Hello, World at root!");

    // GET /api
    let api_route = warp::path("api").map(|| {
        let user = User {
            name: String::from("John"),
            age: 30,
            list: vec![1, 2, 3],
        };

        warp::reply::json(&user)
    });

    // POST /api/dijkstra/:name/:age
    let dijkstra_route = warp::path!("api" / "dijkstra" / String / u8)
    .and(warp::body::json())
    .map(|name:String, age: u8, mut user: User| {
        user.name = format!("{} {}", user.name, name);
        user.age = age;
        warp::reply::json(&user)
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
        .run(([127, 0, 0, 1], 8000))
        .await;
}
