use std::collections::HashMap;
use super::hex::{Hex, HexType};

/// The `HexMap` struct represents a hexagonal grid.
/// A hash map is used to store the hexagons so that no space is wasted in memory.
pub struct HexMap {
    /// The key is (q,r) and the value is the hexagon.
    pub map: HashMap<(i32, i32), Hex>,
}

impl HexMap {
    /// Creates a new `HexMap` with an empty hash map.
    /// # Examples
    /// ```
    /// use server::algorithms::hex_map::HexMap;
    /// let value = HexMap::new();
    /// ```
    pub fn new() -> HexMap {
        return HexMap {
            map: HashMap::new(),
        };
    }

    /// Adds a hexagon to the hash map.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// use server::algorithms::hex_map::HexMap;
    /// let mut value = HexMap::new();
    /// let hex = Hex::new(0, 0, 0, HexType::Empty);
    /// value.add_hex(hex);
    /// ```
    pub fn add_hex(&mut self, hex: Hex) {
        self.map.insert((hex.q, hex.r), hex);
    }

    /// Gets a hexagon from the hash map.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// use server::algorithms::hex_map::HexMap;
    /// let mut value = HexMap::new();
    /// let hex = Hex::new(0, 0, 0, HexType::Empty);
    /// value.add_hex(hex);
    /// let hex_from_map = value.get_hex(0, 0);
    /// ```
    pub fn get_hex(&self, q: i32, r: i32) -> Option<&Hex> {
        return self.map.get(&(q, r));
    }

    /// Returns the size of the hash map.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// use server::algorithms::hex_map::HexMap;
    /// let mut value = HexMap::new();
    /// value.add_hex(Hex::new(0, 0, 0, HexType::Empty));
    /// let size = value.size();
    /// ```
    pub fn size(&self) -> usize {
        return self.map.len();
    }

    /// Returns all the neighbors of a hexagon.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// use server::algorithms::hex_map::HexMap;
    /// let hex1 = Hex::new(0, 0, 0, HexType::Empty);
    /// let hex2 = Hex::new(1, 0, 0, HexType::Empty);
    /// let hex3 = Hex::new(1, 1, 0, HexType::Empty);
    /// let mut map = HexMap::new();
    /// map.add_hex(hex1);
    /// map.add_hex(hex2);
    /// map.add_hex(hex3);
    /// let neighbors = map.get_neighbors(&hex1);
    /// ```
    pub fn get_neighbors(&self, hex: &Hex) -> Vec<&Hex> {
        // create a vector of neighbors
        let mut neighbors: Vec<&Hex> = Vec::new();

        // six directions to check for neighbors in a hexagon
        let hex_direction_vectors = [
            (1, 0, -1),
            (1, -1, 0),
            (0, -1, 1),
            (-1, 0, 1),
            (-1, 1, 0),
            (0, 1, -1),
        ];

        for direction_vector in hex_direction_vectors {
            let neighbor_q = hex.q + direction_vector.0;
            let neighbor_r = hex.r + direction_vector.1;
            let neighbor_s = hex.s + direction_vector.2;
            let neighbor_hex = self.get_hex(neighbor_q, neighbor_r);

            // if the neighbor exists and is not an obstacle, add it to the vector
            match neighbor_hex {
                Some(neighbor_hex) => {
                    if neighbor_hex.s == neighbor_s && neighbor_hex.hex_type != HexType::Obstacle {
                        neighbors.push(neighbor_hex);
                    }
                }
                None => {
                    continue;
                }
            }
        }

        return neighbors;
    }
}