use crate::coordinate::range::Range;

pub mod indexed;
pub mod range;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[repr(C)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    /// Create a new coordinate. Will return an error if the coordinates don't add up to 0.
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self, &'static str> {
        if x + y + z == 0 {
            Ok(Self { x, y, z })
        } else {
            Err("Coordinates do not constitute a valid point")
        }
    }

    /// convert floating point coordinates to the nearest coordinate
    pub fn round(x: f32, y: f32, z: f32) -> Result<Self, &'static str> {
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

    pub fn line_to(&self, end: &Coordinate) -> Range {
        Range::line(self, end)
    }

    pub fn circle(&self, radius: u32) -> Range {
        Range::circle(self, radius)
    }

    pub fn rectangle(&self, to_corner: &Coordinate) -> Range {
        Range::rectangle(self, to_corner)
    }
}

impl From<Offset> for Coordinate {
    fn from(offset: Offset) -> Self {
        match offset {
            Offset { row, column } => {
                let x = column - (row - (row & 1)) / 2;
                let z = row;
                let y = -x - z;
                Coordinate::new(x, y, z).unwrap()
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

