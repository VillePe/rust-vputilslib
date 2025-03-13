#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod polygon;
pub mod rectangle;
pub mod vppoint;

use polygon::Direction;
pub use vppoint::VpPoint;
pub use rectangle::Rectangle;
pub use polygon::Polygon;

/// Calculates the length between two points.
pub fn calc_length_between_points(p1: &VpPoint, p2: &VpPoint) -> f64 {
    calc_length_between(p1.x, p1.y, p2.x, p2.y)
}

/// Calculates the length between two points.
pub fn calc_length_between(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // println!("({x2}-{x1})^2+({y2}-{y1})^2 = {0}", (x2-x1).powf(2.0) + (y2-y1).powf(2.0));
    let temp = (x2-x1).powf(2.0) + (y2-y1).powf(2.0);
    temp.sqrt()
}

/// Gets whether the polygon is clockwise or counterclockwise. Note! Uses the area function
/// and tests if the area is positive or not (at least as of now, might change to something better 
/// at some point). If the area needs to be calculated or has been already calculated, use the 
/// calculate_area function or its value.
/// 
/// If the polygon is self intersecting and the areas negate each other, the direction is set to 
/// CounterClockwise
pub fn get_polygon_direction(polygon: &Polygon) -> Direction {
    let area = calculate_area_internal(polygon);
    if area < 0.0 { Direction::Clockwise } else { Direction::CounterClockwise }
}

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

/// Gets the angle from origo to point. The angle is measured counter clockwise starting from global X-axis
pub fn get_angle_from_points(origo: &VpPoint, p1: &VpPoint) -> f64 {
    get_angle(origo.x, origo.y, p1.x, p1.y)
}

/// Gets the angle from origo to point. The angle is measured counter clockwise starting from global X-axis
pub fn get_angle(origo_x: f64, origo_y: f64, point_x: f64, point_y: f64) -> f64 {
    let x = point_x - origo_x;
    let y = point_y - origo_y;
    let mut add = 0.0;
    if x == 0.0 && y == 0.0 {return 0.0}
    if x == 0.0 {
        let res = if  y > 0.0 {90.0} else { 270.0 };
        return res;
    }
    if y == 0.0 {
        let res = if  x > 0.0 {0.0} else { 180.0 };
        return res;
    }

    let mut radians = (y/x).atan();
    if x < 0.0 && y > 0.0 {
        radians = (-x/y).atan();
        add = 90.0;
    } else if x < 0.0 && y < 0.0 {
        radians = (y/x).atan();
        add = 180.0;
    } else if x > 0.0 && y < 0.0 {
        radians = (-x/y).atan();
        add = 270.0;
    }

    radians.to_degrees() + add
}

/// Normalizes the given angle to be in the range of 0..360
pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized_angle = angle % 360.0;
    if angle < 0.0 {
        normalized_angle += 360.0;
    }
    normalized_angle
}

/// Rotates the given point around the origo. Angle in degrees. Doesn't modify original point.
pub fn rotate_point(origin: &VpPoint, point: &VpPoint, angle: f64) -> VpPoint {
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

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn area_self_intersecting() {
        let mut polygon = Polygon::new_empty();
        polygon.points.push(VpPoint::new(0.0, 0.0));
        polygon.points.push(VpPoint::new(25.0, 25.0));
        polygon.points.push(VpPoint::new(25.0, 0.0));
        polygon.points.push(VpPoint::new(0.0, 25.0));
        polygon.points.push(VpPoint::new(0.0, 0.0));

        let area = calculate_area(&polygon);
        // let centroid = centroid_from_polygon(&polygon);
        let direction = get_polygon_direction(&polygon);
        println!("Direction: {:?}", direction);

        assert_eq!(area, 0.0);
        assert_eq!(direction, Direction::CounterClockwise);
    }

    #[test]
    fn centroid_and_area() {
        // A diamond shape with origo at (0, 0)
        let point1 = VpPoint::new(25.0, 0.0);
        let point2 = VpPoint::new(25.0+25.0, 25.0);
        let point3 = VpPoint::new(25.0, 25.0+25.0);
        let point4 = VpPoint::new(0.0, 25.0);
        let point5 = VpPoint::new(25.0, 0.0);
        let polygon = Polygon::new(vec![point1, point2, point3, point4, point5]);

        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);
        let direction = get_polygon_direction(&polygon);

        assert_eq!(area, 1250.0);
        assert_eq!(centroid.x, 25.0);
        assert_eq!(centroid.y, 25.0);
        assert_eq!(direction, Direction::CounterClockwise);
    }

    #[test]
    fn centroid_clockwise() {
        let mut polygon = Polygon::new_empty();
        // A diamond shape with origo at (0, 0)
        polygon.points.push(VpPoint::new(25.0, 0.0));
        polygon.points.push(VpPoint::new(0.0, 25.0));
        polygon.points.push(VpPoint::new(25.0, 25.0+25.0));
        polygon.points.push(VpPoint::new(25.0+25.0, 25.0));
        polygon.points.push(VpPoint::new(25.0, 0.0));
        
        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);
        let direction = get_polygon_direction(&polygon);
        
        assert_eq!(area, 1250.0);
        assert_eq!(centroid.x, 25.0);
        assert_eq!(centroid.y, 25.0);
        assert_eq!(direction, Direction::Clockwise);
    }

    #[test]
    fn centroid_clockwise_offsetted_from_origo() {
        let mut polygon = Polygon::new_empty();
        // A diamond shape with origo at (50, 10)
        polygon.points.push(VpPoint::new(25.0+50.0, 0.0+10.0)); // 75, 10
        polygon.points.push(VpPoint::new(0.0+50.0, 25.0+10.0)); // 50, 35
        polygon.points.push(VpPoint::new(25.0+50.0, 25.0+25.0+10.0)); // 75, 60
        polygon.points.push(VpPoint::new(25.0+25.0+50.0, 25.0+10.0)); // 100, 35
        polygon.points.push(VpPoint::new(25.0+50.0, 0.0+10.0)); // 75, 10

        let area = calculate_area(&polygon);
        let centroid = centroid_from_polygon(&polygon);
        let direction = get_polygon_direction(&polygon);

        assert_eq!(area, 1250.0);
        assert_eq!(centroid.x, 25.0+50.0);
        assert_eq!(centroid.y, 25.0+10.0);
        assert_eq!(direction, Direction::Clockwise);
    }

    #[test]
    fn calculate_length_between() {
        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(40075000.0, 321321.0);
        let res = calc_length_between_points(&p1, &p2);
        println!("res = {0}", res);
        println!("assert = {0}", (res-40076164.1717409501880));
        assert!((res-40076164.1717409501880).abs() < 0.0001 );

        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(40075000000.0, 321321.0);
        let res = calc_length_between_points(&p1, &p2);
        println!("res = {0}", res);
        println!("assert = {0}", (res-40074999878.287188465614));
        assert!((res-40074999878.287188465614).abs() < 0.0001 );

        let p1 = VpPoint::new(123456.0, 123456.0);
        let p2 = VpPoint::new(40075000000000.0, 321321321.0);
        let res = calc_length_between_points(&p1, &p2);
        println!("res = {0}", res);
        println!("assert = {0}", (res-40074999877831.18738361476462760514));
        assert!((res-40074999877831.18738361476462760514).abs() < 0.0001 );
    }

    #[test]
    fn get_angle() {
        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(123.0+123.0, 123.0+123.0);
        let res = get_angle_from_points(&p1, &p2);
        println!("res1 = {0}", res);
        assert!((res-45.0).abs() < 0.01);

        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(321.0, 555.0);
        let res = get_angle_from_points(&p1, &p2);
        println!("res2 = {0}", res);
        assert!((res-65.3764352138363).abs() < 0.01);

        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(-321.0, 555.0);
        let res = get_angle_from_points(&p1, &p2);
        println!("res3 = {0}", res);
        assert!((res-135.785).abs() < 0.01);

        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(-321.0, -555.0);
        let res = get_angle_from_points(&p1, &p2);
        println!("res4 = {0}", res);
        assert!((res-236.781).abs() < 0.01);

        let p1 = VpPoint::new(123.0, 123.0);
        let p2 = VpPoint::new(321.0, -555.0);
        let res = get_angle_from_points(&p1, &p2);
        println!("res5 = {0}", res);
        assert!((res-286.28).abs() < 0.01);
    }

    #[test]
    fn t_normalize_angle() {
        assert_eq!(normalize_angle(50.0), 50.0);
        assert_eq!(normalize_angle(255.0), 255.0);
        assert_eq!(normalize_angle(-90.0), 270.0);
        assert_eq!(normalize_angle(720.0), 0.0);
        assert_eq!(normalize_angle(715.0), 355.0);
        assert_eq!(normalize_angle(-715.0), 5.0);
    }

}