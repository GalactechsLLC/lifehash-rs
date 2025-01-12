use crate::colors::gradient::select_gradient;
use crate::grids::cell_grid::Cellgrid;
use crate::grids::change_grid::ChangeGrid;
use crate::grids::color_grid::ColorGrid;
use crate::grids::frac_grid::FracGrid;
use crate::utils::bits::Enumerator;
use crate::utils::{clamped, lerp_from, select_pattern};
use crate::{Dimensions, Image, Version};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::io::{Error, ErrorKind};

fn make_image(
    width: usize,
    height: usize,
    float_colors: Vec<f64>,
    module_size: usize,
    has_alpha: bool,
) -> Result<Image, Error> {
    if module_size == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid Module Size"));
    }
    let scaled_width = width * module_size;
    let scaled_height = height * module_size;
    let result_components = if has_alpha { 4 } else { 3 };
    let scaled_capacity = scaled_width * scaled_height * result_components;
    let mut result_colors = vec![0u8; scaled_capacity];
    for target_y in 0..scaled_width {
        for target_x in 0..scaled_height {
            let source_x = target_x / module_size;
            let source_y = target_y / module_size;
            let source_offset = (source_y * width + source_x) * 3;
            let target_offset = (target_y * scaled_width + target_x) * result_components;
            result_colors[target_offset] = (clamped(float_colors[source_offset]) * 255.0) as u8;
            result_colors[target_offset + 1] =
                (clamped(float_colors[source_offset + 1]) * 255.0) as u8;
            result_colors[target_offset + 2] =
                (clamped(float_colors[source_offset + 2]) * 255.0) as u8;
            if has_alpha {
                result_colors[target_offset + 3] = 255;
            }
        }
    }
    Ok(Image {
        width: scaled_width,
        height: scaled_height,
        channels: if has_alpha { 4 } else { 3 },
        pixels: result_colors,
    })
}

pub fn from_data(
    data: &[u8],
    version: Version,
    module_size: usize,
    has_alpha: bool,
) -> Result<(Image, Vec<u8>), Error> {
    let sha256 = Sha256::digest(data);
    from_digest(sha256.as_slice(), version, module_size, has_alpha)
}

pub fn from_digest(
    digest: &[u8],
    version: Version,
    module_size: usize,
    has_alpha: bool,
) -> Result<(Image, Vec<u8>), Error> {
    let (length, max_generations) = match version {
        Version::Version1 | Version::Version2 => (16, 150),
        Version::Detailed | Version::Fiducial | Version::GrayscaleFiducial => (32, 300),
    };
    let dimensions = Dimensions {
        width: length,
        height: length,
    };
    // These get reused from generation to generation by swapping them.
    let mut current_cell_grid = Cellgrid::new(dimensions);
    let mut next_cell_grid = Cellgrid::new(dimensions);
    let mut current_change_grid = ChangeGrid::new(dimensions);
    let mut next_change_grid = ChangeGrid::new(dimensions);
    let mut history_set = HashSet::new();
    let mut history: Vec<Vec<u8>> = Vec::default();
    match version {
        Version::Version1 => {
            next_cell_grid.set_data(digest);
        }
        Version::Version2 => {
            let digest = Sha256::digest(digest);
            // Ensure that .version2 in no way resembles .version1
            next_cell_grid.set_data(&digest);
        }
        Version::Detailed | Version::Fiducial | Version::GrayscaleFiducial => {
            // Ensure that GRAYSCALE fiducials in no way resemble the regular color fiducials
            let mut digest1 = if version == Version::GrayscaleFiducial {
                Sha256::digest(digest).to_vec()
            } else {
                digest.to_vec()
            };
            let digest2 = Sha256::digest(&digest1);
            let digest3 = Sha256::digest(digest2);
            let digest4 = Sha256::digest(digest3);
            digest1.extend(digest2);
            digest1.extend(digest3);
            digest1.extend(digest4);
            next_cell_grid.set_data(&digest1);
        }
    }
    next_change_grid.set_all(true);

    while history.len() < max_generations {
        (current_cell_grid, next_cell_grid) = (next_cell_grid, current_cell_grid);
        (current_change_grid, next_change_grid) = (next_change_grid, current_change_grid);
        let data = current_cell_grid.get_data();
        if history_set.contains(&data) {
            break;
        }
        history_set.insert(data.clone());
        history.push(data);
        current_cell_grid.next_generation(
            &current_change_grid,
            &mut next_cell_grid,
            &mut next_change_grid,
        );
    }
    let mut frac_grid = FracGrid::new(dimensions);
    for (i, item) in history.iter().enumerate() {
        current_cell_grid.set_data(item.as_slice());
        let frac = clamped(lerp_from(0.0, history.len() as f64, i as f64 + 1.0));
        frac_grid.overlay(&current_cell_grid, frac);
    }

    // Normalizing the frac_grid to the range 0..1 was a step left out of .version1
    // In some cases it can cause the full range of the gradient to go unused.
    // This fixes the problem for the other versions, while remaining compatible
    // with .version1.
    if version != Version::Version1 {
        let mut min_value = f64::INFINITY;
        let mut max_value = f64::NEG_INFINITY;
        for point in frac_grid.get_points() {
            let value = frac_grid.get_value(point);
            min_value = min_value.min(*value);
            max_value = max_value.max(*value);
        }
        for point in frac_grid.get_points() {
            let current = frac_grid.get_value(point);
            let value = lerp_from(min_value, max_value, *current);
            frac_grid.set_value(value, point);
        }
    }
    let mut entropy = Enumerator::new(digest);
    match version {
        Version::Detailed => {
            // Throw away a bit of entropy to ensure we generate different colors and patterns from .version1
            let _ = entropy.next()?;
        }
        Version::Version2 => {
            // Throw away two bits of entropy to ensure we generate different colors and patterns from .version1 or .detailed.
            let _ = entropy.next()?;
            let _ = entropy.next()?;
        }
        _ => {}
    }
    let gradient = select_gradient(&mut entropy, version)?;
    let pattern = select_pattern(&mut entropy, version)?;
    let color_grid = ColorGrid::create(frac_grid, gradient, pattern);
    make_image(
        color_grid.dimensions.width as usize,
        color_grid.dimensions.height as usize,
        color_grid.colors(),
        module_size,
        has_alpha,
    )
    .map(|i| (i, digest.to_vec()))
}
