use crate::{ColorValues, Dimensions, Point};
use std::marker::PhantomData;

pub mod cell_grid;
pub mod change_grid;
pub mod color_grid;
pub mod frac_grid;

pub struct Grid<T, C> {
    pub dimensions: Dimensions,
    pub max_x: usize,
    pub max_y: usize,
    pub storage: Vec<T>,
    _phantom_data: PhantomData<C>,
}
impl<T: Default + Clone, C: ColorValues<T>> Grid<T, C> {
    pub fn new(dimensions: Dimensions) -> Grid<T, C> {
        let max_x = dimensions.width - 1;
        let max_y = dimensions.height - 1;
        let storage = vec![T::default(); dimensions.width * dimensions.height];
        Self {
            dimensions,
            max_x,
            max_y,
            storage,
            _phantom_data: PhantomData,
        }
    }
    fn offset(&self, point: Point) -> usize {
        point.y * self.dimensions.width + point.x
    }
    fn circular_index(index: i32, modulus: usize) -> usize {
        (index + modulus as i32) as usize % modulus
    }
    pub fn set_all(&mut self, value: T) {
        self.storage.fill(value);
    }
    pub fn set_value(&mut self, value: T, point: Point) {
        let index = self.offset(point);
        self.storage[index] = value;
    }
    pub fn get_value(&self, point: Point) -> &T {
        let index = self.offset(point);
        &self.storage[index]
    }
    pub fn get_points(&self) -> Vec<Point> {
        let mut points = vec![];
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                points.push(Point { x, y });
            }
        }
        points
    }
    pub fn get_neighborhood(&self, point: Point) -> Vec<(bool, Point)> {
        let mut points = vec![];
        for oy in -1..2 {
            for ox in -1..2 {
                let px = Self::circular_index(ox + point.x as i32, self.dimensions.width);
                let py = Self::circular_index(oy + point.y as i32, self.dimensions.height);
                points.push((ox == 0 && oy == 0, Point { x: px, y: py }));
            }
        }
        points
    }

    pub fn colors(&self) -> Vec<f64> {
        let mut result = vec![];
        for value in &self.storage {
            let c = C::color_for_value(value);
            result.push(c.r);
            result.push(c.g);
            result.push(c.b);
        }
        result
    }
}
