use serde::{Serialize, Deserialize};

pub mod indexed;
pub mod range;
mod faces;
mod dist;

#[derive(Debug, Copy, Clone)]
pub struct World {
    x: i32,
    y: i32,
    tile_width: u8,
    tile_height: u8,
}

impl World {
    pub fn new(x: i32, y: i32, tile_width: u8, tile_height: u8) -> Self {
        World {
            x,
            y,
            tile_height,
            tile_width,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Offset {
    pub row: i32,
    pub column: i32,
}

impl Offset {
    pub fn new(column: i32, row: i32) -> Self {
        Offset { column, row }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

pub const ZERO: Coordinate = Coordinate::new(0, 0, 0);

impl Coordinate {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        if x + y + z == 0 {
            Self { x, y, z }
        } else {
            Self { x: 0, y: 0, z: 0 }
        }
    }

    /// convert floating point coordinates to the nearest coordinate
    pub fn round(x: f64, y: f64, z: f64) -> Self {
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();
        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }
        Self::new(rx as i32, ry as i32, rz as i32)
    }
}

impl From<Offset> for Coordinate {
    fn from(offset: Offset) -> Self {
        match offset {
            Offset { row, column } => {
                let x = column - (row - (row & 1)) / 2;
                let z = row;
                let y = -x - z;
                Coordinate::new(x, y, z)
            }
        }
    }
}

impl From<&Coordinate> for Offset {
    fn from(coordinate: &Coordinate) -> Self {
        match coordinate {
            Coordinate { x, y: _, z } => {
                let column = x + (z - (z & 1)) / 2;
                let row = *z;
                Offset { column, row }
            }
        }
    }
}

impl From<Coordinate> for Offset {
    fn from(coordinate: Coordinate) -> Self {
        coordinate.into()
    }
}

impl From<World> for Coordinate {
    fn from(world: World) -> Self {
        //todo remove
        assert!(world.x >= 0 && world.y >= 0 && world.tile_width == world.tile_height, "simple for now");

        let part_width: f32 = world.tile_width as f32;
        let part_height: f32 = 1.5 * world.tile_height as f32;
        let part_x: i32 = (world.x as f32 / part_width).floor() as i32;
        let part_y: i32 = (world.y as f32 / part_height).floor() as i32;
        let part_ix = world.x as f32 % part_width;
        let part_iy = world.y as f32 % part_height;
        let row = 2 * part_y;
        if part_iy > world.tile_height as f32 {
            //below main body
            return if part_ix < world.tile_width as f32 / 2. {
                //left
                Offset::new(part_x - 1, row + 1).into()
            } else {
                //right
                Offset::new(part_x, row + 1).into()
            };
        }
        if part_iy <= world.tile_height as f32 / 4. {
            //upper overflow
            return if part_ix < world.tile_width as f32 / 2. {
                //left
                if part_ix / 2. <= part_iy {
                    Offset::new(part_x - 1, row - 1).into()
                } else {
                    Offset::new(part_x, row).into()
                }
            } else {
                //right
                if part_ix / 2. > part_iy {
                    Offset::new(part_x, row - 1).into()
                } else {
                    Offset::new(part_x, row).into()
                }
            };
        }
        if part_iy > world.tile_height as f32 / 4. && part_iy <= world.tile_height as f32 {
            //main body
            return Offset::new(part_x, row).into();
        }
        //lower overflow
        return if part_ix < world.tile_width as f32 / 2. {
            //left
            if part_ix / 2. <= part_iy as f32 {
                Offset::new(part_x - 1, row + 1).into()
            } else {
                Offset::new(part_x, row).into()
            }
        } else {
            //right
            if part_ix / 2. > part_iy as f32 {
                Offset::new(part_x, row + 1).into()
            } else {
                Offset::new(part_x, row).into()
            }
        };
    }
}

impl std::ops::Add<&Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Coordinate) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
        Coordinate::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
        Coordinate::round((self.x as f64) * scale, (self.y as f64) * scale, (self.z as f64) * scale)
    }
}

impl std::ops::Mul<f64> for Coordinate {
    type Output = Coordinate;

    fn mul(self, scale: f64) -> Self::Output {
        (&self) * scale
    }
}

