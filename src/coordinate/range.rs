use crate::coordinate::{Coordinate, Offset, ZERO};
use std::collections::HashSet;
use std::iter::FromIterator;

pub type Range = HashSet<Coordinate>;

const DIRECTIONS: [Coordinate; 6] = [
    Coordinate::new(1, -1),
    Coordinate::new(1, 0),
    Coordinate::new(0, 1),
    Coordinate::new(-1, 1),
    Coordinate::new(-1, 0),
    Coordinate::new(0, -1),
];

pub trait RangeFactory {
    fn new(coordinates: &[Coordinate]) -> Range {
        Range::from_iter(coordinates.into_iter().cloned())
    }

    fn neighbors(coordinate: &Coordinate) -> Range {
        if *coordinate == ZERO {
            return Range::from_iter(DIRECTIONS.iter().cloned());
        }
        Range::from_iter(DIRECTIONS.iter().map(|d| coordinate + d))
    }

    fn ring(center: &Coordinate, radius: u16) -> Range {
        let mut results = vec![];
        let i_radius = radius as i32;
        for x in -i_radius..=i_radius {
            for y in (-i_radius).max(-x - i_radius)..=i_radius.min(-x + i_radius) {
                results.push(Coordinate::new(x, y) + *center)
            }
        }
        Range::from_iter(results)
    }

    fn ring0(radius: u16) -> Range {
        Self::ring(&Default::default(), radius)
    }

    fn circle(center: &Coordinate, radius: u16) -> Range {
        let mut results = vec![];
        for sub_radius in 0..=radius {
            results.extend(Range::ring(center, sub_radius).into_iter())
        }
        Range::from_iter(results)
    }

    fn circle0(radius: u16) -> Range {
        Self::circle(&Default::default(), radius)
    }

    fn line(_start: &Coordinate, _end: &Coordinate) -> Range {
        unimplemented!()
    }

    fn line0(end: &Coordinate) -> Range {
        Self::line(&Default::default(), end)
    }

    fn rectangle(from_corner: &Coordinate, to_corner: &Coordinate) -> Range {
        let Offset {
            row: row_from,
            column: column_from,
        } = from_corner.into();
        let Offset {
            row: row_to,
            column: column_to,
        } = to_corner.into();
        Range::from_iter(
            (row_from..=row_to)
                .flat_map(move |row| {
                    (column_from..=column_to)
                        .map(move |column| -> Coordinate { Offset::new(column, row).into() })
                })
                .into_iter(),
        )
    }

    fn rectangle0(to_corner: &Coordinate) -> Range {
        Self::rectangle(&Default::default(), to_corner)
    }
}

impl RangeFactory for Range {}

pub trait RangeFrom {
    fn line_to(&self, end: &Coordinate) -> Range;
    fn circle(&self, radius: u16) -> Range;
    fn ring(&self, radius: u16) -> Range;
    fn rectangle_to(&self, to_corner: &Coordinate) -> Range;
}

impl RangeFrom for Coordinate {
    fn line_to(&self, end: &Coordinate) -> Range {
        Range::line(self, end)
    }

    fn circle(&self, radius: u16) -> Range {
        Range::circle(self, radius)
    }

    fn ring(&self, radius: u16) -> Range {
        Range::ring(self, radius)
    }

    fn rectangle_to(&self, to_corner: &Coordinate) -> Range {
        Range::rectangle(self, to_corner)
    }
}
