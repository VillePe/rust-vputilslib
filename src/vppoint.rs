#![allow(dead_code)]

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


#[cfg(test)]
mod tests {
    use crate::vppoint::VpPoint;

    #[test]
    fn new() {
        let point = VpPoint::new(1.0, 2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        println!("{:?}", point);
    }

}