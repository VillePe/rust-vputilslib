use serde::{Deserialize, Serialize};

use crate::geometry2d::{get_max_x, get_max_y, get_min_x, get_min_y, Polygon, VpPoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    /// The bottom left point of the rectangle
    pub origin: VpPoint,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {width, height, origin: VpPoint::new(x, y)}
    }

    pub fn new_vppoint(bottom_left: VpPoint, width: f64, height: f64) -> Self {
        Self {width, height, origin: bottom_left}
    }
}

/// Gets the bounding box from given polygon. Polygon needs atleast one point to calculate the 
/// bounding box. With single point the returned rectangles width and height will be set to 0.0001 
pub fn bounding_box(polygon: &Polygon) -> Option<Rectangle> {
    if polygon.points.is_empty() {return None;}
    if polygon.points.len() == 1 {
        let point = polygon.points.first().unwrap();
        return Some(Rectangle::new(point.x, point.y, 0.0001, 0.0001))
    }
    let min_x = get_min_x(&polygon.points).unwrap();
    let max_x = get_max_x(&polygon.points).unwrap();
    let min_y = get_min_y(&polygon.points).unwrap();
    let max_y = get_max_y(&polygon.points).unwrap();
    Some(Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y))
}

#[cfg(test)]
mod tests {
    use crate::geometry2d::{Polygon, Rectangle, VpPoint};
    use crate::geometry2d::rectangle::*;

    #[test]
    fn rectangle() {
        let rectangle = Rectangle::new(0.0, 0.0, 100.0, 200.0);
        assert_eq!(rectangle.origin, VpPoint::new(0.0, 0.0));
        assert_eq!(rectangle.width, 100.0);
        assert_eq!(rectangle.height, 200.0);
    }

    #[test]
    fn rectangle_vppoint() {
        let rectangle = Rectangle::new_vppoint(VpPoint::new(0.0, 0.0), 100.0, 200.0);
        assert_eq!(rectangle.origin, VpPoint::new(0.0, 0.0));
        assert_eq!(rectangle.width, 100.0);
        assert_eq!(rectangle.height, 200.0);
    }
    
    #[test]
    fn bounding_box_test() {
        let mut polygon = Polygon::new_empty();
        polygon.points.push(VpPoint::new(0.0+10.0, 0.0+10.0));
        polygon.points.push(VpPoint::new(25.0+10.0, 25.0+10.0));
        polygon.points.push(VpPoint::new(25.0+10.0, 0.0+10.0));
        polygon.points.push(VpPoint::new(0.0+10.0, 25.0+10.0));
        polygon.points.push(VpPoint::new(0.0+10.0, 0.0+10.0));
        
        let bb = bounding_box(&polygon).unwrap();
        assert_eq!(bb.origin.x, 10.0);
        assert_eq!(bb.origin.y, 10.0);
        assert_eq!(bb.width, 25.0);
        assert_eq!(bb.height, 25.0);
    }
}