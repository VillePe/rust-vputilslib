#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::geometry2d::polygon::Polygon;

pub mod polygon;
pub mod rectangle;

#[derive(Debug)]
pub struct VpPoint {
    pub x: f64,
    pub y: f64,
}

impl VpPoint {
    pub fn new(x: f64, y: f64) -> VpPoint {
        VpPoint { x, y }
    }
}

/// Calculates the area of given polygon. Always returns a positive value.
pub fn calculate_area(polygon: &Polygon) -> f64 {
    let area = calculate_area_internal(polygon);
    area.abs()
}

/// Calculates the area of given polygon. If the points in polygon are in counterclockwise order,
/// a positive value is returned. Otherwise the returned value is negative. See more at
/// https://en.wikipedia.org/wiki/Shoelace_formula Triangle formula
fn calculate_area_internal(polygon: &Polygon) -> f64 {
    let mut area = 0.0;
    for i in 0..(polygon.points.len()-1) {
        area += area_sum_function(&polygon.points[i], &polygon.points[i+1]);
    }
    area += area_sum_function(&polygon.points[polygon.points.len()-1], &polygon.points[0]);
    area /= 2.0;

    area
}

/// See https://en.wikipedia.org/wiki/Shoelace_formula Triangle formula
fn area_sum_function(p1: &VpPoint, p2: &VpPoint) -> f64 {
    p1.x * p2.y - p2.x * p1.y
}

/// Calculates the centroid from given polygon from current origo (positive axes: x →, y ↑)
pub fn centroid_from_polygon(polygon: &Polygon) -> VpPoint {
    let (mut x, mut y) = (0.0, 0.0);
    for i in 0..polygon.points.len()-1 {
        let cur_x = polygon.points[i].x;
        let next_x = polygon.points[i+1].x;
        let cur_y = polygon.points[i].y;
        let next_y = polygon.points[i+1].y;
        x += (cur_x + next_x)*(cur_x*next_y - next_x*cur_y);
        y += (cur_y+next_y)*(cur_x*next_y-next_x*cur_y);
    }

    let area = calculate_area_internal(polygon);

    x /= 6.0*area;
    y /= 6.0*area;

    VpPoint::new(x, y)
}

#[cfg(test)]
mod tests {
    use crate::geometry2d::polygon::Polygon;
    use crate::geometry2d::{calculate_area, centroid_from_polygon, VpPoint};

    #[test]
    fn centroid() {
        let point1 = VpPoint::new(25.0, 0.0);
        let point2 = VpPoint::new(25.0+25.0, 25.0);
        let point3 = VpPoint::new(25.0, 25.0+25.0);
        let point4 = VpPoint::new(0.0, 25.0);
        let point5 = VpPoint::new(25.0, 0.0);
        let polygon = Polygon::new(vec![point1, point2, point3, point4, point5]);

        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);

        assert_eq!(area, 1250.0);
        assert_eq!(centroid.x, 25.0);
        assert_eq!(centroid.y, 25.0);
    }

    #[test]
    fn centroid_clockwise() {
        let mut polygon = Polygon::new_empty();
        polygon.points.push(VpPoint::new(25.0, 0.0));
        polygon.points.push(VpPoint::new(0.0, 25.0));
        polygon.points.push(VpPoint::new(25.0, 25.0+25.0));
        polygon.points.push(VpPoint::new(25.0+25.0, 25.0));
        polygon.points.push(VpPoint::new(25.0, 0.0));
        
        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);
        
        assert_eq!(area, 1250.0);
        assert_eq!(centroid.x, 25.0);
        assert_eq!(centroid.y, 25.0);
    }

    #[test]
    fn centroid_clockwise_offsetted_from_origo() {
        let mut polygon = Polygon::new_empty();
        polygon.points.push(VpPoint::new(25.0+50.0, 0.0+10.0));
        polygon.points.push(VpPoint::new(0.0+50.0, 25.0+10.0));
        polygon.points.push(VpPoint::new(25.0+50.0, 25.0+25.0+10.0));
        polygon.points.push(VpPoint::new(25.0+25.0+50.0, 25.0+10.0));
        polygon.points.push(VpPoint::new(25.0+50.0, 0.0+10.0));

        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);

        assert_eq!(area, 1250.0);
        assert_eq!(centroid.x, 25.0+50.0);
        assert_eq!(centroid.y, 25.0+10.0);
    }

}