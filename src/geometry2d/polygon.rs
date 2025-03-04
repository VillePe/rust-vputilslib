use serde::{Deserialize, Serialize};

use crate::geometry2d::VpPoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<VpPoint>
}

impl Polygon {
    pub fn new(points: Vec<VpPoint>) -> Polygon {
        Polygon { points }
    }
    pub fn new_empty() -> Polygon {
        Polygon { points: vec![] }
    }
}