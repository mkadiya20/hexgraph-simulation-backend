pub mod algorithms;
use algorithms::{dijkstra, floyd_warshall};

pub fn test_lib() {
    println!("called `test_lib()`");

    dijkstra::test_dijkstra();
    floyd_warshall::test_floyd_warshall();
}