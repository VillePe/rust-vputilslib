use crate::geometry2d::VpPoint;

pub struct Polygon {
    pub points: Vec<VpPoint>
}

impl Polygon {
    pub fn new(points: Vec<VpPoint>) -> Polygon {
        Polygon { points: points }
    }
    pub fn new_empty() -> Polygon {
        Polygon { points: vec![] }
    }
}