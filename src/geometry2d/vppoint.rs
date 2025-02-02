#[derive(Debug, PartialEq)]
pub struct VpPoint {
    pub x: f64,
    pub y: f64,
}

impl VpPoint {
    pub fn new(x: f64, y: f64) -> VpPoint {
        VpPoint { x, y }
    }
}