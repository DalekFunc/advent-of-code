#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Coord3D {
    pub fn xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}
