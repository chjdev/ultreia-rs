use crate::coordinate::Coordinate;
use crate::coordinate::dist::Dist;

#[repr(C)]
pub enum Face {
    Left,
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    None,
}

pub trait Faces {
    fn touching_face_between(c1: &Coordinate, c2: &Coordinate) -> Face {
        if c1.dist(c2) != 1 {
            return Face::None;
        }
        let dist = c1 - c2;
        return if dist.x == 0 {
            if dist.y < 0 {
                Face::BottomLeft
            } else {
                Face::TopRight
            }
        } else if dist.x < 0 {
            if dist.y == 0 {
                Face::BottomRight
            } else {
                Face::Right
            }
        } else if dist.y == 0 {
            Face::TopLeft
        } else {
            Face::Left
        };
    }

    fn touching_face(&self, other: &Coordinate) -> Face;
}

impl Faces for Coordinate {
    fn touching_face(&self, other: &Coordinate) -> Face {
        Self::touching_face_between(self, other)
    }
}