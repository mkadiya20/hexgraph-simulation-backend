use std::collections::HashMap;
use super::hex::Hex;

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
}