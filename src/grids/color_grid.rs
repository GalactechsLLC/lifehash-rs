use crate::colors::functions::ColorFunction;
use crate::colors::rgb::Color;
use crate::grids::frac_grid::FracGrid;
use crate::grids::Grid;
use crate::utils::target_size;
use crate::{ColorValues, Pattern, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Transform {
    transpose: bool,
    reflect_x: bool,
    reflect_y: bool,
}

pub const SNOWFLAKE_TRANSFORMS: [Transform; 4] = [
    Transform {
        transpose: false,
        reflect_x: false,
        reflect_y: false,
    },
    Transform {
        transpose: false,
        reflect_x: true,
        reflect_y: false,
    },
    Transform {
        transpose: false,
        reflect_x: false,
        reflect_y: true,
    },
    Transform {
        transpose: false,
        reflect_x: true,
        reflect_y: true,
    },
];

pub const PINWHEEL_TRANSFORMS: [Transform; 4] = [
    Transform {
        transpose: false,
        reflect_x: false,
        reflect_y: false,
    },
    Transform {
        transpose: true,
        reflect_x: true,
        reflect_y: false,
    },
    Transform {
        transpose: true,
        reflect_x: false,
        reflect_y: true,
    },
    Transform {
        transpose: false,
        reflect_x: true,
        reflect_y: true,
    },
];

pub const FIDUCIAL_TRANSFORMS: [Transform; 1] = [Transform {
    transpose: false,
    reflect_x: false,
    reflect_y: false,
}];

pub type ColorGrid = Grid<Color, ColorGridImpl>;
pub struct ColorGridImpl;
impl ColorGrid {
    pub fn create(frac_grid: &FracGrid, gradient: &ColorFunction, pattern: Pattern) -> Self {
        let mut color_grid = ColorGrid::new(target_size(frac_grid.dimensions, pattern));
        let transforms = match pattern {
            Pattern::Snowflake => SNOWFLAKE_TRANSFORMS.as_slice(),
            Pattern::Pinwheel => PINWHEEL_TRANSFORMS.as_slice(),
            Pattern::Fiducial => FIDUCIAL_TRANSFORMS.as_slice(),
        };
        for point in frac_grid.get_points() {
            let value = *frac_grid.get_value(point);
            let color = gradient.apply(value);
            color_grid.draw(point, color, transforms);
        }
        color_grid
    }
    pub fn transform_point(&self, point: Point, transform: Transform) -> Point {
        let mut result = point;
        if transform.transpose {
            (result.x, result.y) = (result.y, result.x);
        }
        if transform.reflect_x {
            result.x = self.max_x - result.x;
        }
        if transform.reflect_y {
            result.y = self.max_y - result.y;
        }
        result
    }
    pub fn draw(&mut self, point: Point, color: Color, transforms: &[Transform]) {
        for t in transforms {
            let p2 = self.transform_point(point, *t);
            self.set_value(color, p2);
        }
    }
}
impl ColorValues<Color> for ColorGridImpl {
    fn color_for_value(value: &Color) -> Color {
        *value
    }
}
