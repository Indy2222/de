use glam::Vec2;

pub struct Aabb {
    min: Vec2,
    max: Vec2,
}

impl Aabb {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn intersects(&self, rhs: &Self) -> bool {
        self.min.cmple(rhs.max).all() && self.max.cmpge(rhs.min).all()
    }
}
