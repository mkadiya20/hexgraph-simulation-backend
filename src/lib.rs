pub mod algorithms;
use algorithms::{hex::{Hex,HexType}, hex_map::HexMap, dijkstra};

/// Converts an offset coord to a hex coord.
/// # Examples
/// ```
/// use server::offset_to_hex;
/// use server::algorithms::hex::{Hex, HexType};
/// let offset_coord = [-1, 1];
/// let hex = offset_to_hex(offset_coord[0], offset_coord[1], HexType::Empty);
/// ```
pub fn offset_to_hex(row: i32, col: i32, hex_type: HexType) -> Hex {
    let q = col - (row - (row&1)) / 2;
    let r = row;
    let s = -q - r;
    return Hex::new(q, r, s, hex_type);
}

/// Creates a new `HexMap` with the given 2D vector graph.
/// # Examples
/// ```
/// use server::create_hex_map;
/// use server::algorithms::hex::{Hex, HexType};
/// let graph = vec![
///         vec!['b', 'b', 'b'],
///         vec!['b', 'b', 'b'],
///         vec!['b', 'b', 'b']
///     ];
/// let hex_map = create_hex_map(graph);
pub fn create_hex_map(graph: Vec<Vec<char>>) -> Result<HexMap, &'static str> {
    let mut hex_map = HexMap::new();
    for (row, row_vec) in graph.iter().enumerate() {
        for (col, col_char) in row_vec.iter().enumerate() {
            let hex_type = match col_char {
                'b' => HexType::Empty,
                'o' => HexType::Obstacle,
                's' => HexType::Start,
                'e' => HexType::End,
                _ => return Err("Invalid character in graph"),
            };
            let hex = offset_to_hex(row as i32, col as i32, hex_type);
            hex_map.add_hex(hex);
        }
    }
    return Ok(hex_map);
}

/// Handles the given request, performing the appropriate algorithm and returning the result.
pub fn handle_request(
    request_type: String,
    source: [i32; 2],
    target: [i32; 2],
    graph: Vec<Vec<char>>
) -> Vec<String> {
    let hex_map = create_hex_map(graph);
    match hex_map {
        Ok(hex_map) => {
            let source_hex = offset_to_hex(source[0], source[1], HexType::Start);
            let target_hex = offset_to_hex(target[0], target[1], HexType::End);
            match request_type.as_str() {
                "dijkstra" => {
                    let path = dijkstra::find_path(&hex_map, &source_hex, &target_hex);

                    // convert the path to a vector of strings
                    match path {
                        Ok(path) => {
                            return path.iter()
                                .map(|hex| hex.to_string())
                                .collect();
                        }
                        Err(err) => {
                            return vec![err.to_string()];
                        }
                    }
                }
                _ => {
                    return vec!["Not valid".to_string()];
                }
            }
        }
        Err(err) => return vec![err.to_string()],
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::{
            hex::{Hex, HexType},
            hex_map::HexMap
        },
        offset_to_hex,
        create_hex_map, handle_request,
    };

    #[test]
    fn test_hex() {
        let value: Hex = Hex::new(0, 0, 0, HexType::Empty);
        assert_eq!(value.q, 0);
        assert_eq!(value.r, 0);
        assert_eq!(value.s, 0);
    }

    #[test]
    fn test_empty_hex_map() {
        let value: HexMap = HexMap::new();
        assert_eq!(value.size(), 0);
    }

    #[test]
    fn test_hex_map_add_hex() {
        let mut value: HexMap = HexMap::new();
        let hex: Hex = Hex::new(0, 0, 0, HexType::Empty);
        value.add_hex(hex);
        assert_eq!(value.size(), 1);
    }

    #[test]
    fn test_hex_map_get_hex() {
        let mut value: HexMap = HexMap::new();
        let hex: Hex = Hex::new(0, 0, 0, HexType::Empty);
        value.add_hex(hex);
        let hex_from_map: Option<&Hex> = value.get_hex(0, 0);
        match hex_from_map {
            Some(hex_from_map) => {
                assert_eq!(hex_from_map.q, 0);
                assert_eq!(hex_from_map.r, 0);
                assert_eq!(hex_from_map.s, 0);
            }
            None => {
                panic!("Hex not found in map");
            }
        }
    }

    #[test]
    fn test_hex_map_get_invalid_hex() {
        let mut value: HexMap = HexMap::new();
        let hex: Hex = Hex::new(0, 0, 0, HexType::Empty);
        value.add_hex(hex);
        let hex_from_map: Option<&Hex> = value.get_hex(1, 1);
        match hex_from_map {
            Some(_) => {
                panic!("Hex found in map");
            }
            None => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_hex_to_offset() {
        let hex: Hex = Hex::new(1, 2, -3, HexType::Empty);
        let offset: (i32, i32) = hex.to_offset_coord();
        assert_eq!(offset.0, 2);
        assert_eq!(offset.1, 2);
    }

    #[test]
    fn test_offset_to_hex() {
        let value: Hex = offset_to_hex(-2, 2, HexType::Empty);
        assert_eq!(value.q, 3);
        assert_eq!(value.r, -2);
        assert_eq!(value.s, -1);
    }

    #[test]
    fn test_create_hex_map() {
        let value: HexMap = create_hex_map(vec![
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b']
        ]).unwrap();
        assert_eq!(value.size(), 9);
    }

    #[test]
    fn test_create_hex_map_invalid_char() {
        let value = create_hex_map(vec![
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'i']
        ]);

        match value {
            Ok(_) => {
                panic!("Invalid character in graph");
            }
            Err(err) => {
                assert_eq!(err, "Invalid character in graph");
            }
        }
    }

    #[test]
    fn test_get_neighbors() {
        let hex_map = create_hex_map(vec![
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b']
        ]).unwrap();
        let hex = hex_map.get_hex(1, 1).unwrap();
        let neighbors = hex_map.get_neighbors(hex);
        assert_eq!(neighbors.len(), 6);
    }

    #[test]
    fn test_get_corner_neighbors() {
        let hex_map = create_hex_map(vec![
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'b']
        ]).unwrap();
        let hex = hex_map.get_hex(1, 2).unwrap();
        let neighbors = hex_map.get_neighbors(hex);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn test_dijkstra_algorithm_no_obstacles() {
        let graph = vec![
            vec!['s', 'b', 'b'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'e']
        ];
        let path = handle_request("dijkstra".to_string(), [0,0], [2,2], graph);
        println!("{:?}", path);

        assert_eq!(path[0], "Hex(1,2,-3)");
        assert_eq!(path[1], "Hex(0,2,-2)");
        assert_eq!(path[2], "Hex(0,1,-1)");
        assert_eq!(path[3], "Hex(0,0,0)");
    }

    #[test]
    fn test_dijkstra_algorithm_with_obstacles() {
        let graph = vec![
            vec!['s', 'b', 'b'],
            vec!['o', 'o', 'b'],
            vec!['o', 'o', 'e']
        ];
        let path = handle_request("dijkstra".to_string(), [0,0], [2,2], graph);
        println!("{:?}", path);

        assert_eq!(path[0], "Hex(1,2,-3)");
        assert_eq!(path[1], "Hex(2,1,-3)");
        assert_eq!(path[2], "Hex(2,0,-2)");
        assert_eq!(path[3], "Hex(1,0,-1)");
        assert_eq!(path[4], "Hex(0,0,0)");
    }

    #[test]
    fn test_dijkstra_algorithm_no_path() {
        let graph = vec![
            vec!['s', 'b', 'b'],
            vec!['o', 'o', 'o'],
            vec!['o', 'o', 'e']
        ];
        let path = handle_request("dijkstra".to_string(), [0,0], [2,2], graph);
        println!("{:?}", path);

        assert_eq!(path[0], "No path found");
    }
}