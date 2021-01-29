use crate::coordinate::Coordinate;

pub trait Dist {
    /// manhatten dist
    fn dist_between(a: &Coordinate, b: &Coordinate) -> u32 {
        let dx = (a.x.max(b.x) - a.x.min(b.x)) as u32;
        let dy = (a.y.max(b.y) - a.y.min(b.y)) as u32;
        let dz = (a.z().max(b.z()) - a.z().min(b.z())) as u32;
        dx.max(dy).max(dz)
    }

    fn dist(&self, other: &Coordinate) -> u32;
}

impl Dist for Coordinate {
    fn dist(&self, other: &Coordinate) -> u32 {
        Self::dist_between(self, other)
    }
}
