/// The `Hex` struct represents a hexagon in a hexagonal grid.
/// This struct uses the cube coordinate system.
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Hex {
    /// Creates a new `Hex` with the given `q`, `r`, and `s` values.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::Hex;
    /// let hex = Hex::new(0, 0, 0);
    /// ```
    pub fn new(q: i32, r: i32, s: i32) -> Hex {
        return Hex {
            q: q,
            r: r,
            s: s,
        }
    }

    /// Converts the `Hex` to its corresponding offset coord on a 2D grid.
    /// # Examples
    /// ```
    /// use server::algorithms::hex::Hex;
    /// let hex = Hex::new(0, 0, 0);
    /// let offset = hex.to_offset_coord();
    /// ```
    pub fn to_offset_coord(&self) -> (i32, i32) {
        let row = self.r;
        let col = self.q + (self.r - (self.r&1)) / 2;
        return (row, col);
    }
}