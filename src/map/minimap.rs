use crate::coordinate::range::Range;
use crate::coordinate::{Coordinate, Offset};

pub trait GetByCoordinate<T> {
    fn get(&self, coordinate: &Coordinate) -> T;
    fn get_range(&self, range: &Range) -> Vec<T> {
        range
            .into_iter()
            .map(|coordinate| self.get(coordinate))
            .collect()
    }
}

pub trait SetByCoordinate<T> {
    fn set(&mut self, coordinate: Coordinate, value: T);
    fn set_range<F>(&mut self, range: Range, gen_value: F)
    where
        F: Fn(&Coordinate) -> T,
    {
        for coordinate in range {
            let value = gen_value(&coordinate);
            self.set(coordinate, value);
        }
    }
}

pub trait TrySetByCoordinate<T>: SetByCoordinate<T> + GetByCoordinate<T> {
    fn try_set(&mut self, coordinate: Coordinate, value: T) -> bool;
    fn try_set_range<F>(&mut self, range: Range, gen_value: F) -> bool
    where
        F: Fn(&Coordinate) -> T;
}

impl<S, T> TrySetByCoordinate<Option<S>> for T
where
    T: SetByCoordinate<Option<S>> + GetByCoordinate<Option<S>>,
{
    fn try_set(&mut self, coordinate: Coordinate, value: Option<S>) -> bool {
        if self.get(&coordinate).is_some() {
            return false;
        }
        self.set(coordinate, value);
        true
    }

    fn try_set_range<F>(&mut self, range: Range, gen_value: F) -> bool
    where
        F: Fn(&Coordinate) -> Option<S>,
    {
        for coordinate in range.iter() {
            if self.get(coordinate).is_some() {
                return false;
            }
        }
        for coordinate in range.into_iter() {
            let value = gen_value(&coordinate);
            self.set(coordinate, value);
        }
        true
    }
}

pub trait FillByCoordinate<T: Copy>: SetByCoordinate<T> {
    fn fill(&mut self, range: Range, value: T) {
        self.set_range(range, |_| value);
    }
}

pub trait FillClonedByCoordinate<T: Clone>: SetByCoordinate<T> {
    fn fill_cloned(&mut self, range: Range, value: T) {
        self.set_range(range, |_| value.clone());
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
