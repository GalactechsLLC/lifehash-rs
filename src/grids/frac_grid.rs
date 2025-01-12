use crate::colors::{rgb::Color, BLACK, WHITE};
use crate::grids::cell_grid::Cellgrid;
use crate::grids::Grid;
use crate::ColorValues;

pub type FracGrid = Grid<f64, FracGridImpl>;
pub struct FracGridImpl;
impl FracGrid {
    pub fn overlay(&mut self, cell_grid: &Cellgrid, frac: f64) {
        for p in self.get_points() {
            if *cell_grid.get_value(p) {
                self.set_value(frac, p);
            }
        }
    }
}
impl ColorValues<f64> for FracGridImpl {
    fn color_for_value(value: &f64) -> Color {
        BLACK.lerp_to(&WHITE, *value)
    }
}
