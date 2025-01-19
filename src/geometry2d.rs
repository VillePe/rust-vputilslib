#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod polygon;
pub mod rectangle;
mod vppoint;

/// Calculates the area of given polygon. Always returns a positive value.
/// Note, polygon can't self intersect or the calculated area is not correct.
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

/// Gets the minimum x value from given list of points
pub fn get_min_x(points: &Vec<VpPoint>) -> Option<f64> {
    if points.len() == 0 {
        return None;
    }
    let mut min = points[0].x;
    for p in points {
        if p.x < min {
            min = p.x;
        }
    }
    Some(min)
}

/// Gets the minimum x value from given list of points
pub fn get_max_x(points: &Vec<VpPoint>) -> Option<f64> {
    if points.len() == 0 {
        return None;
    }
    let mut max = points[0].x;
    for p in points {
        if p.x > max {
            max = p.x;
        }
    }
    Some(max)
}

/// Gets the minimum x value from given list of points
pub fn get_min_y(points: &Vec<VpPoint>) -> Option<f64> {
    if points.len() == 0 {
        return None;
    }
    let mut min = points[0].y;
    for p in points {
        if p.y < min {
            min = p.y;
        }
    }
    Some(min)
}

/// Gets the minimum x value from given list of points
pub fn get_max_y(points: &Vec<VpPoint>) -> Option<f64> {
    if points.len() == 0 {
        return None;
    }
    let mut max = points[0].y;
    for p in points {
        if p.y > max {
            max = p.y;
        }
    }
    Some(max)
}

/// Rotates the given point around the origo. Angle in degrees. Doesn't modify original point.
pub fn rotate_point(origin: VpPoint, point: VpPoint, angle: f64) -> VpPoint {
    let (x,y) = rotate(origin.x, origin.y, point.x, point.y, angle);
    VpPoint::new(x, y)
}

/// Rotates the point around given origo. Angle in degrees. 
pub fn rotate(origin_x: f64, origin_y: f64, x: f64, y: f64, angle: f64) -> (f64, f64) {
    let degrees_in_radians = angle.to_radians();
    let c = degrees_in_radians.cos();
    let s = degrees_in_radians.sin();
    let offset_x = x - origin_x;
    let offset_y = y - origin_y;
    let new_x = offset_x * c - offset_y * s;
    let new_y = offset_x * s + offset_y * c;
    
    (origin_x+new_x, origin_y+new_y)
}

// The structs are defined below. Implementations and traits are handled in geometry2d folder

#[derive(Debug, PartialEq)]
pub struct VpPoint {
    pub x: f64,
    pub y: f64,
}

pub struct Polygon {
    pub points: Vec<VpPoint>
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    /// The bottom left point of the rectangle
    pub origin: VpPoint,
}

#[cfg(test)]
mod tests {
    use crate::geometry2d::Polygon;
    use crate::geometry2d::{calculate_area, centroid_from_polygon, VpPoint};
    
    

    #[test]
    fn area_self_intersecting() {
        let mut polygon = Polygon::new_empty();
        polygon.points.push(VpPoint::new(0.0, 0.0));
        polygon.points.push(VpPoint::new(25.0, 25.0));
        polygon.points.push(VpPoint::new(25.0, 0.0));
        polygon.points.push(VpPoint::new(0.0, 25.0));
        polygon.points.push(VpPoint::new(0.0, 0.0));

        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);

        assert_eq!(area, 0.0);
    }

    #[test]
    fn centroid_and_area() {
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