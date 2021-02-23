use std::collections::HashMap;

use crate::coordinate::Coordinate;

pub type CoordinateIndexed<T> = HashMap<Coordinate, T>;
