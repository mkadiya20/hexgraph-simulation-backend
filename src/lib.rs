pub mod algorithms;
use algorithms::{dijkstra, floyd_warshall};

pub fn test_lib() -> String {
    dijkstra::test_dijkstra();
    floyd_warshall::test_floyd_warshall();
    return String::from("lib");
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{dijkstra, floyd_warshall, hex::Hex, hex_map::HexMap};

    #[test]
    fn test_dijkstra() {
        let value: String = dijkstra::test_dijkstra();
        assert_eq!(value, "dijkstra");
    }

    #[test]
    fn test_floyd_warshall() {
        let value: String = floyd_warshall::test_floyd_warshall();
        assert_eq!(value, "floyd_warshall");
    }

    #[test]
    fn test_hex() {
        let value: Hex = Hex::new(0, 0, 0);
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
        let hex: Hex = Hex::new(0, 0, 0);
        value.add_hex(hex);
        assert_eq!(value.size(), 1);
    }

    #[test]
    fn test_hex_map_get_hex() {
        let mut value: HexMap = HexMap::new();
        let hex: Hex = Hex::new(0, 0, 0);
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
        let hex: Hex = Hex::new(0, 0, 0);
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
}