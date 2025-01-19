use crate::geometry2d::*;

impl Polygon {
    pub fn new(points: Vec<VpPoint>) -> Polygon {
        Polygon { points }
    }
    pub fn new_empty() -> Polygon {
        Polygon { points: vec![] }
    }
}