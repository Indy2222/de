use de_core::objects::ObjectType;
use de_uom::Metre;
use parry2d::{math::Point, shape::ConvexPolygon};

use crate::{loader::Footprint, ObjectCache};

/// Padding around static object ichnographies used to accommodate for moving
/// object trajectory smoothing and non-zero moving object sizes.
pub const EXCLUSION_OFFSET: f32 = Metre::new_unchecked(2.);

pub trait IchnographyCache {
    fn get_ichnography(&self, object_type: ObjectType) -> &Ichnography;
}

impl IchnographyCache for ObjectCache {
    fn get_ichnography(&self, object_type: ObjectType) -> &Ichnography {
        self.get(object_type).ichnography()
    }
}

pub struct Ichnography {
    offset_convex_hull: ConvexPolygon,
}

impl Ichnography {
    pub fn new(offset_convex_hull: ConvexPolygon) -> Self {
        Self { offset_convex_hull }
    }

    pub fn offset_convex_hull(&self) -> &ConvexPolygon {
        &self.offset_convex_hull
    }
}

impl From<&Footprint> for Ichnography {
    fn from(footprint: &Footprint) -> Self {
        let footprint = ConvexPolygon::from_convex_polyline(
            footprint
                .convex_hull()
                .iter()
                .map(|&[x, y]| Point::new(x, y))
                .collect(),
        )
        .unwrap();

        Self::new(footprint.offsetted(EXCLUSION_OFFSET))
    }
}
