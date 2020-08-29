use crate::coordinate::{Coordinate, ZERO};

pub struct Range(Vec<Coordinate>);

const DIRECTIONS: [Coordinate; 6] = [
    Coordinate::new(1, -1, 0),
    Coordinate::new(1, 0, -1),
    Coordinate::new(0, 1, -1),
    Coordinate::new(-1, 1, 0),
    Coordinate::new(-1, 0, 1),
    Coordinate::new(0, -1, 1),
];

// todo remove
#[allow(unused_variables)]
impl Range {
    pub fn new(coordinates: Vec<Coordinate>) -> Self {
        Range(coordinates)
    }

    pub fn neighbors(coordinate: &Coordinate) -> Self {
        if *coordinate == ZERO {
            return Self(DIRECTIONS.to_vec());
        }
        Range(DIRECTIONS.iter().map(|d| {
            coordinate + d
        }).collect())
    }

    pub fn circle(center: &Coordinate, radius: u16) -> Self {
        let mut results = vec![];
        let i_radius = radius as i32;
        for x in -i_radius..=i_radius {
            let lower = (-i_radius).max(-x - i_radius);
            let upper = (i_radius).max(-x + i_radius);
            for y in lower..=upper {
                let z = -x - y;
                let offset = Coordinate::new(x, y, z);
                results.push(center + offset);
            }
        }
        Self(results)
    }

    pub fn circle0(radius: u16) -> Self {
        Self::circle(&Default::default(), radius)
    }

    pub fn line(start: &Coordinate, end: &Coordinate) -> Self {
        unimplemented!()
    }

    pub fn line0(end: &Coordinate) -> Self {
        Self::line(&Default::default(), end)
    }

    pub fn rectangle(from_corner: &Coordinate, to_corner: &Coordinate) -> Self {
        unimplemented!()
    }

    pub fn rectangle0(to_corner: &Coordinate) -> Self {
        Self::rectangle(&Default::default(), to_corner)
    }
}

pub trait RangeFrom {
    fn line_to(&self, end: &Coordinate) -> Range;
    fn circle(&self, radius: u16) -> Range;
    fn rectangle_to(&self, to_corner: &Coordinate) -> Range;
}

impl RangeFrom for Coordinate {
    fn line_to(&self, end: &Coordinate) -> Range {
        Range::line(self, end)
    }

    fn circle(&self, radius: u16) -> Range {
        Range::circle(self, radius)
    }

    fn rectangle_to(&self, to_corner: &Coordinate) -> Range {
        Range::rectangle(self, to_corner)
    }
}

impl IntoIterator for Range {
    type Item = Coordinate;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl<'a> IntoIterator for &'a Range {
    type Item = &'a Coordinate;
    type IntoIter = std::slice::Iter<'a, Coordinate>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
//
// impl<'a> IntoIterator for &'a mut Range {
//     type Item = &'a mut Coordinate;
//     type IntoIter = std::slice::IterMut<'a, Coordinate>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter_mut()
//     }
// }
