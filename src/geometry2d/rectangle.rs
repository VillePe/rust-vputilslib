use crate::geometry2d::*;

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {width, height, origin: VpPoint::new(x, y)}
    }

    pub fn new_vppoint(bottom_left: VpPoint, width: f64, height: f64) -> Self {
        Self {width, height, origin: bottom_left}
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry2d::{Rectangle, VpPoint};

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
}