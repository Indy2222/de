use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    eq::AlmostEq,
    utils::{orientation, Orientation},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LineSegment(pub Vec2, pub Vec2);

impl LineSegment {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        // TODO document panic

        if a.almost_eq(&b) {
            panic!("Points a {:?} and b {:?} are too close together.", a, b);
        }
        Self(a, b)
    }

    pub fn normal(&self) -> Vec2 {
        (self.0 - self.1).perp()
    }

    pub fn intersects(&self, rhs: &Self) -> bool {
        let a = orientation(self.0, self.1, rhs.0);
        let b = orientation(self.0, self.1, rhs.1);
        if a == Orientation::Collinear && b == Orientation::Collinear {
            let dir = self.1 - self.0;
            let dir_square = dir.length_squared();

            let proj_a = (rhs.0 - self.0).dot(dir);
            let proj_b = (rhs.1 - self.0).dot(dir);
            return (0. <= proj_a && proj_a <= dir_square)
                || (0. <= proj_b && proj_b <= dir_square);
        }

        let c = orientation(rhs.0, rhs.1, self.0);
        let d = orientation(rhs.0, rhs.1, self.1);
        a != b && c != d
    }
}

impl AlmostEq for LineSegment {
    fn almost_eq(&self, rhs: &Self) -> bool {
        self.0.almost_eq(&rhs.0) && self.1.almost_eq(&rhs.1)
    }
}

pub struct LineSegmentIterator<'a> {
    vertices: &'a [Vec2],
    index: usize,
}

impl<'a> LineSegmentIterator<'a> {
    pub fn new(vertices: &'a [Vec2]) -> Self {
        Self { vertices, index: 0 }
    }
}

impl<'a> Iterator for LineSegmentIterator<'a> {
    type Item = LineSegment;

    fn next(&mut self) -> Option<Self::Item> {
        let second_index = self.index + 1;
        if second_index >= self.vertices.len() {
            return None;
        }
        let a = self.vertices[self.index];
        self.index = second_index;
        Some(LineSegment(a, self.vertices[self.index]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let segment = LineSegment::new(Vec2::new(0., 0.), Vec2::new(1., 0.));
        let normal = segment.normal();
        assert!(normal.almost_eq(&Vec2::new(0., -1.)));
    }

    #[test]
    fn test_iterator() {
        let vertices = vec![Vec2::new(0., 1.), Vec2::new(0., 2.), Vec2::new(0., 3.)];
        let mut segments = LineSegmentIterator::new(&vertices);
        assert!(segments
            .next()
            .unwrap()
            .almost_eq(&LineSegment::new(Vec2::new(0., 1.), Vec2::new(0., 2.))));
        assert!(segments
            .next()
            .unwrap()
            .almost_eq(&LineSegment::new(Vec2::new(0., 2.), Vec2::new(0., 3.))));
        assert!(segments.next().is_none());
        assert!(segments.next().is_none());
    }
}
