use std::collections::HashMap;

use crate::coordinate::Coordinate;

pub type Indexed<T> = HashMap<Coordinate, T>;
