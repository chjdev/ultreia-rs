use crate::coordinate::Coordinate;

pub struct Range(Vec<Coordinate>);

impl Range {
    pub fn new(coordinates: Vec<Coordinate>) -> Self {
        Range(coordinates)
    }

    pub fn circle(coordinate: &Coordinate, radius: u32) -> Self {
        unimplemented!()
    }

    pub fn circle0(radius: u32) -> Self {
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
