use serde::{Serialize, Deserialize};
use crate::coordinate::Coordinate;
use crate::server::Range;

#[derive(Serialize, Deserialize)]
pub struct WithCoordinate<T: Serialize>(Coordinate, T);

impl<T: Serialize> WithCoordinate<T> {
    pub fn new(coordinate: Coordinate, value: T) -> Self {
        WithCoordinate(
            coordinate,
            value,
        )
    }

    pub fn index(values: impl IntoIterator<Item=T>, range: &Range) -> Vec<WithCoordinate<T>> {
        values.into_iter().zip(range)
            .map(|(value, coordinate)|
                WithCoordinate::new(*coordinate, value))
            .collect::<Vec<WithCoordinate<T>>>()
    }
}