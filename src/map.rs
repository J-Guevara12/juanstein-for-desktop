use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use crate::{HEIGHT, WIDTH};

const MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 1, 0, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

const BACKGROUND_COLOR: Rgb666 = Rgb666::WHITE;
const LINE_COLOR: Rgb666 = Rgb666::BLACK;

pub fn draw_map<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb666>,
{
    let grid_size: i32 =
        std::cmp::min(WIDTH as usize / MAP.len(), HEIGHT as usize / MAP[0].len()) as i32;
    let space_style = PrimitiveStyleBuilder::new()
        .stroke_color(LINE_COLOR)
        .stroke_width(1)
        .fill_color(BACKGROUND_COLOR)
        .build();

    let wall_style = PrimitiveStyleBuilder::new()
        .stroke_color(LINE_COLOR)
        .stroke_width(1)
        .fill_color(LINE_COLOR)
        .build();

    for (y, row) in MAP.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            Rectangle::new(
                Point::new(x as i32 * grid_size, y as i32 * grid_size),
                Size::new(grid_size as u32 + 1, grid_size as u32 + 1),
            )
            .into_styled(if *value == 0 { space_style } else { wall_style })
            .draw(display)?
        }
    }
    Ok(())
}
