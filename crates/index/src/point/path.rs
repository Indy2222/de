use glam::Vec2;

// TOD docs
pub(super) struct BinaryPath {
    axis: u8,
    min: [f32; 2],
    max: [f32; 2],
    target: [f32; 2],
}

impl BinaryPath {
    pub(super) fn new(half_size: Vec2, target: Vec2) -> Self {
        Self {
            axis: 0,
            min: (-half_size).to_array(),
            max: half_size.to_array(),
            target: target.to_array(),
        }
    }

    pub(super) fn step(&mut self) -> Half {
        let axis = self.axis as usize;
        self.axis ^= 1;

        let mid = 0.5 * (self.min[axis] + self.max[axis]);
        if self.target[axis] < mid {
            self.max[axis] = mid;
            Half::TopLeft
        } else {
            self.min[axis] = mid;
            Half::BottomRight
        }
    }
}

// TODO docs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Half {
    TopLeft,
    BottomRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let mut path = BinaryPath::new(Vec2::new(2., 3.), Vec2::new(1., 0.2));

        // right
        assert_eq!(path.step(), Half::BottomRight);
        // bottom
        assert_eq!(path.step(), Half::BottomRight);
        // left
        assert_eq!(path.step(), Half::TopLeft);
        // top
        assert_eq!(path.step(), Half::TopLeft);
        // left
        assert_eq!(path.step(), Half::TopLeft);
        // top
        assert_eq!(path.step(), Half::TopLeft);
    }
}
