/// The `HexType` enum represents the type of a hexagon.
/// `Empty` is the default type, it is traversible.
/// `Obstacle` represents a hexagon that cannot be traversed.
/// `Start` represents the starting hexagon.
/// `End` represents the ending hexagon.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HexType {
    Empty,
    Obstacle,
    Start,
    End,
}

/// The `Hex` struct represents a hexagon in a hexagonal grid.
/// This struct uses the cube coordinate system.
/// Each `Hex` has a state that defines its type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub hex_type: HexType
}

impl Hex {
    /// Creates a new `Hex` with the given `q`, `r`, and `s` values.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// let hex = Hex::new(0, 0, 0, HexType::Empty);
    /// ```
    pub fn new(q: i32, r: i32, s: i32, hex_type: HexType) -> Hex {
        return Hex {
            q,
            r,
            s,
            hex_type
        }
    }

    /// Converts the `Hex` to its corresponding offset coord on a 2D grid.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// let hex = Hex::new(0, 0, 0, HexType::Empty);
    /// let offset = hex.to_offset_coord();
    /// ```
    pub fn to_offset_coord(&self) -> (i32, i32) {
        let row = self.r;
        let col = self.q + (self.r - (self.r&1)) / 2;
        return (row, col);
    }

    /// Returns the string representation of the `Hex`.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::{Hex, HexType};
    /// let hex = Hex::new(0, 0, 0, HexType::Empty);
    /// let string = hex.to_string();
    /// ```
    pub fn to_string(&self) -> String {
        return format!("Hex({},{},{})", self.q, self.r, self.s);
    }
}