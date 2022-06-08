use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{aabb::Aabb, utils::Orientation};
use crate::{segment::LineSegmentIterator, utils::orientation};

#[derive(Serialize, Deserialize)]
pub struct Polygon {
    exterior: Vec<Vec2>,
}

impl Polygon {
    pub fn aabb(&self) -> Aabb {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for point in self.exterior.iter() {
            min_x = min_x.min(point.x);
            min_y = min_x.min(point.y);
            max_x = min_x.max(point.x);
            max_y = min_x.max(point.y);
        }
        Aabb::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
    }

    pub fn intersects(&self, rhs: &Polygon) -> bool {
        // Handle rsh fully inside self case.
        let point = rhs.exterior[0];
        if LineSegmentIterator::new(self.exterior.as_slice())
            .all(|segment| orientation(segment.0, segment.1, point) != Orientation::Cw)
        {
            return true;
        }

        // Handle self fully inside rhs case
        let point = self.exterior[0];
        if LineSegmentIterator::new(rhs.exterior.as_slice())
            .all(|segment| orientation(segment.0, segment.1, point) != Orientation::Cw)
        {
            return true;
        }

        for line_a in LineSegmentIterator::new(self.exterior.as_slice()) {
            for line_b in LineSegmentIterator::new(self.exterior.as_slice()) {
                if line_a.intersects(&line_b) {
                    return true;
                }
            }
        }

        false
    }
}
