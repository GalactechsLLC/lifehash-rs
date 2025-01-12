use crate::colors::{rgb::Color, BLACK, WHITE};
use crate::grids::change_grid::ChangeGrid;
use crate::grids::Grid;
use crate::utils::bits::{Aggregator, Enumerator};
use crate::{ColorValues, Point, ZERO};

pub type Cellgrid = Grid<bool, CellGridImpl>;
pub struct CellGridImpl;
impl Cellgrid {
    pub fn get_data(&self) -> Vec<u8> {
        let mut a = Aggregator::new();
        for point in self.get_points() {
            a.append(*self.get_value(point));
        }
        a.data
    }
    pub fn set_data(&mut self, data: &[u8]) {
        let mut e = Enumerator::new(data);
        let mut i = 0;
        while let Ok(b) = e.next() {
            self.storage[i] = b;
            i += 1;
        }
    }
    pub fn is_alive_in_next_generation(current_alive: bool, neighbors_count: usize) -> bool {
        if current_alive {
            neighbors_count == 2 || neighbors_count == 3
        } else {
            neighbors_count == 3
        }
    }
    pub fn count_neighbors(&self, point: Point) -> usize {
        let mut total = 0;
        for (o, p) in self.get_neighborhood(point) {
            if o == ZERO {
                continue;
            }
            if *self.get_value(p) {
                total += 1;
            }
        }
        total
    }
    pub fn next_generation(
        &mut self,
        current_change_grid: &ChangeGrid,
        next_cell_grid: &mut Self,
        next_change_grid: &mut ChangeGrid,
    ) {
        next_cell_grid.set_all(false);
        next_change_grid.set_all(false);
        for p in self.get_points() {
            let current_alive = *self.get_value(p);
            if *current_change_grid.get_value(p) {
                let neighbors_count = self.count_neighbors(p);
                let next_alive = Self::is_alive_in_next_generation(current_alive, neighbors_count);
                if next_alive {
                    next_cell_grid.set_value(true, p);
                }
                if current_alive != next_alive {
                    next_change_grid.set_changed(p);
                }
            } else {
                next_cell_grid.set_value(current_alive, p);
            }
        }
    }
}
impl ColorValues<bool> for CellGridImpl {
    fn color_for_value(value: &bool) -> Color {
        if *value {
            WHITE
        } else {
            BLACK
        }
    }
}
