use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;

use super::hex::{Hex};
use super::hex_map::HexMap;

/// Returns the shortest path from `source` to `target` using Dijkstra's algorithm.
/// If no path is found, an error is returned.
/// # Examples
/// ```
/// use server::algorithms::dijkstra::find_path;
/// use server::algorithms::hex::{Hex, HexType};
/// use server::algorithms::hex_map::HexMap;
/// let mut hex_map = HexMap::new();
/// let source_hex = Hex::new(0, 0, 0, HexType::Empty);
/// let target_hex = Hex::new(1, 1, 1, HexType::Empty);
/// hex_map.add_hex(source_hex);
/// hex_map.add_hex(target_hex);
/// let path = find_path(&hex_map, &source_hex, &target_hex);
/// ```
pub fn find_path(hex_map: &HexMap, source: &Hex, target: &Hex) -> Result<Vec<Hex>, &'static str> {
    // store the distance from the source to each hex
    let mut distances = HashMap::new();

    // store each hex that has been visited
    let mut visited = HashSet::new();

    // store all hexes that need to be visited in a heap
    let mut to_visit = BinaryHeap::new();

    // store the parent of each hex to allow backtracking and reconstructing the path
    let mut parent = HashMap::new();

    // add the source hex and set its distance to 0
    distances.insert(source, 0);
    parent.insert(source, None);

    // add the source hex to the heap of hexes to visit
    to_visit.push(Visit {
        vertex: source,
        distance: 0
    });

    // graph is unweighted, undirected so cost of moving from one hex to another is 1
    let cost = 1;

    // while there are still hexes to visit
    while let Some(Visit {vertex,distance}) = to_visit.pop() {
        // if the hex has already been visited, continue
        if !visited.insert(vertex) {
            continue;
        }

        // get the neighbours of the current vertex
        let neighbors = hex_map.get_neighbors(vertex);

        // for each neighbour, if it hasn't been visited, update its distance and add it to the heap
        for neighbor in neighbors {
            let new_distance = distance + cost;
            let is_shorter = distances
                .get(neighbor)
                .map_or(true, |&current| new_distance < current);
            
            // if the distance is shorter, update it and set its parent
            if is_shorter {
                distances.insert(neighbor, new_distance);
                parent.insert(neighbor, Some(vertex));
                to_visit.push(Visit {
                    vertex: neighbor,
                    distance: new_distance
                });
            }
        }
    }

    // if the target is not reachable, return an error
    if !distances.contains_key(target) {
        return Err("No path found".into());
    }

    // reconstruct the path by backtracking from the target to the source
    let result = generate_path(&parent, target);

    return Ok(result);
}

/// Generates the path from the target to the source by backtracking from the target to the source.
fn generate_path(parent: &HashMap<&Hex, Option<&Hex>>, target: &Hex) -> Vec<Hex> {
    let mut target_hex = target;
    let mut result_list = Vec::new();

    result_list.push(*target_hex);

    while let Some(next_hex) = parent.get(target_hex) {
        match next_hex {
            Some(next_hex) => {
                target_hex = next_hex;
            }
            None => {
                return result_list;
            }
        }
        result_list.push(*target_hex);
    }

    return result_list;
}

#[derive(Debug)]
struct Visit<T> {
    vertex: T,
    distance: i32
}

impl<T> Ord for Visit<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<T> PartialOrd for Visit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Visit<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<T> Eq for Visit<T> {}