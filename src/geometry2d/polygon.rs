use serde::{Deserialize, Serialize};

use crate::geometry2d::{self, VpPoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<VpPoint>,
}

impl Polygon {
    pub fn new(points: Vec<VpPoint>) -> Polygon {
        Polygon { points }
    }
    pub fn new_empty() -> Polygon {
        Polygon { points: vec![] }
    }

    pub fn get_line(&self, index: i32) -> Option<(&VpPoint, &VpPoint)> {
        let p1;
        let p2;
        if index >= self.points.len() as i32 {
            return None
        } else {
            p1 = &self.points[index as usize];
            p2 = &self.points[index as usize + 1];
        }
        Some((p1, p2))
    }

    pub fn get_line_or_last(&self, index: i32) -> (&VpPoint, &VpPoint) {
        let p1;
        let p2;
        if index >= self.points.len() as i32 {
            // Get the last line of the polygon if the index is out of bounds
            p1 = &self.points[self.points.len() - 2];
            p2 = &self.points[self.points.len() - 1];
        } else {
            p1 = &self.points[index as usize];
            p2 = &self.points[index as usize + 1];
        }
        (p1, p2)
    }

    pub fn get_direction(&self) -> Direction {
        geometry2d::get_polygon_direction(self)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Clockwise = -1,
    CounterClockwise = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_line_or_last() {
        let p1 = VpPoint::new(0.0, 0.0);
        let p2 = VpPoint::new(1.0, 0.0);
        let p3 = VpPoint::new(1.0, 1.0);
        let p4 = VpPoint::new(0.0, 1.0);
        let p5 = VpPoint::new(0.0, 0.0);
        let polygon = Polygon::new(vec![p1, p2, p3, p4, p5]);
        assert_eq!(
            polygon.get_line_or_last(0),
            (&polygon.points[0], &polygon.points[1])
        );
        assert_eq!(
            polygon.get_line_or_last(1),
            (&polygon.points[1], &polygon.points[2])
        );
        assert_eq!(
            polygon.get_line_or_last(2),
            (&polygon.points[2], &polygon.points[3])
        );
        assert_eq!(
            polygon.get_line_or_last(3),
            (&polygon.points[3], &polygon.points[4])
        );
        assert_eq!(
            polygon.get_line_or_last(6),
            (&polygon.points[3], &polygon.points[4])
        );
        assert_eq!(
            polygon.get_line_or_last(150),
            (&polygon.points[3], &polygon.points[4])
        );
    }
}
