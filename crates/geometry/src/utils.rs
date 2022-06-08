use glam::Vec2;

use crate::eq::AlmostEq;

#[derive(Debug, Eq, PartialEq)]
pub enum Orientation {
    Cw,
    Ccw,
    Collinear,
}

pub(crate) fn orientation(a: Vec2, b: Vec2, c: Vec2) -> Orientation {
    let perp_a = a.perp_dot(c);
    let perp_b = b.perp_dot(c);

    if perp_a.almost_eq(&perp_b) {
        Orientation::Collinear
    } else if perp_a < perp_b {
        Orientation::Ccw
    } else {
        Orientation::Cw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccw() {
        assert_eq!(
            orientation(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(2.1, 1.)),
            Orientation::Ccw
        );
        assert_eq!(
            orientation(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(-2.1, 1.)),
            Orientation::Ccw
        );
        assert_eq!(
            orientation(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(2.1, -1.)),
            Orientation::Cw
        );
        assert_eq!(
            orientation(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(-2.1, -1.)),
            Orientation::Cw
        );
    }
}
