use approx::ulps_eq;
use glam::Vec2;

pub trait AlmostEq {
    fn almost_eq(&self, rsh: &Self) -> bool;
}

impl AlmostEq for f32 {
    fn almost_eq(&self, rhs: &Self) -> bool {
        ulps_eq!(*self, *rhs)
    }
}

impl AlmostEq for Vec2 {
    fn almost_eq(&self, rhs: &Self) -> bool {
        ulps_eq!(self.x, rhs.x) && ulps_eq!(self.y, rhs.y)
    }
}
