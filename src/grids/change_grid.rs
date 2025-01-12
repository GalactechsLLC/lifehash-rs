use crate::colors::{rgb::Color, BLUE, RED};
use crate::grids::Grid;
use crate::{ColorValues, Point};

pub type ChangeGrid = Grid<bool, ChangeGridImpl>;
pub struct ChangeGridImpl;
impl ChangeGrid {
    pub fn set_changed(&mut self, point: Point) {
        for (_, p) in self.get_neighborhood(point) {
            self.set_value(true, p);
        }
    }
}
impl ColorValues<bool> for ChangeGridImpl {
    fn color_for_value(value: &bool) -> Color {
        if *value {
            RED
        } else {
            BLUE
        }
    }
}
