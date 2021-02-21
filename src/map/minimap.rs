use crate::coordinate::range::Range;
use crate::coordinate::{Coordinate, Offset};

pub trait GetByCoordinate<T> {
    fn get(&self, coordinate: &Coordinate) -> T;
    fn range(&self, range: &Range) -> Vec<T> {
        range.into_iter().map(|c| self.get(c)).collect()
    }
}

pub trait WithGrid {
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
}

pub trait Minimap<T>: GetByCoordinate<T> + WithGrid {
    fn minimap(&self, width: u16, height: u16) -> Vec<T> {
        // with_capacity does not work in godot context for some reason
        let mut minimap: Vec<T> = Vec::new();
        let scale_x = self.columns() as f64 / (width as f64);
        let scale_y = self.rows() as f64 / (height as f64);
        let height_half = (height / 2) as i16;
        let width_half = (width / 2) as i16;
        for y in -height_half..height_half {
            let row_id = (y + height_half) as usize * width as usize;
            let row = (y as f64 * scale_y) as i32;
            for x in -width_half..width_half {
                let idx = row_id + (x + width_half) as usize;
                let column = (x as f64 * scale_x) as i32;
                let coordinate: Coordinate = Offset::new(column, row).into();
                minimap.insert(idx as usize, self.get(&coordinate));
            }
        }
        minimap
    }
}