mod dist;
mod faces;
pub mod indexed;
pub mod range;

#[derive(Debug, Copy, Clone)]
pub struct Offset {
    row: i32,
    column: i32,
}

impl Offset {
    pub fn new(column: i32, row: i32) -> Self {
        Offset { column, row }
    }

    pub fn row(&self) -> i32 {
        self.row
    }
    pub fn column(&self) -> i32 {
        self.column
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

pub const ZERO: Coordinate = Coordinate::new(0, 0);

impl Coordinate {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// convert floating point coordinates to the nearest coordinate
    pub fn round(x: f64, y: f64) -> Self {
        let mut rx = x.round();
        let mut ry = y.round();
        let rz = -rx - ry;

        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        if x_diff > y_diff {
            rx = -ry - rz;
        } else {
            ry = -rx - rz;
        }
        Self::new(rx as i32, ry as i32)
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }
    pub fn y(&self) -> i32 {
        return self.x;
    }
    pub fn z(&self) -> i32 {
        return -self.x - self.y;
    }
}

impl From<Offset> for Coordinate {
    fn from(offset: Offset) -> Self {
        match offset {
            Offset { row, column } => {
                let x = column;
                let z = row - (column + (column & 1)) / 2;
                let y = -x - z;
                Coordinate::new(x, y)
            }
        }
    }
}

impl From<&Coordinate> for Offset {
    fn from(coordinate: &Coordinate) -> Self {
        let x = coordinate.x();
        let z = coordinate.z();
        let column = x;
        let row = z + (x + (x & 1)) / 2;
        Offset { column, row }
    }
}

impl From<Coordinate> for Offset {
    fn from(coordinate: Coordinate) -> Self {
        coordinate.into()
    }
}

impl std::ops::Add<&Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Coordinate) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Add<Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        self + (&rhs)
    }
}

impl std::ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        (&self) + (&rhs)
    }
}

impl std::ops::Sub<&Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: &Coordinate) -> Self::Output {
        Coordinate::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Sub<Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        (self) - (&rhs)
    }
}

impl std::ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        (&self) - (&rhs)
    }
}

impl std::ops::Mul<f64> for &Coordinate {
    type Output = Coordinate;

    fn mul(self, scale: f64) -> Self::Output {
        Coordinate::round((self.x as f64) * scale, (self.y as f64) * scale)
    }
}

impl std::ops::Mul<f64> for Coordinate {
    type Output = Coordinate;

    fn mul(self, scale: f64) -> Self::Output {
        (&self) * scale
    }
}
