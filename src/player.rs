use std::time::Instant;

use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

pub struct Player {
    px: f32,
    py: f32,
    theta: f32,
    last_time_moved: Instant,
}

const PLAYER_SIZE: u32 = 3;
const PLAYER_SPEED: f32 = 4.0;
const PLAYER_COLOR: Rgb666 = Rgb666::CSS_AQUA;

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        return Player {
            px: x,
            py: y,
            theta: 0.0,
            last_time_moved: Instant::now(),
        };
    }

    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb666>,
    {
        Rectangle::new(
            Point::new(
                (self.px.round() as u32 - PLAYER_SIZE / 2) as i32,
                (self.py.floor() as u32 - PLAYER_SIZE / 2) as i32,
            ),
            Size::new(PLAYER_SIZE, PLAYER_SIZE),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(PLAYER_COLOR)
                .build(),
        )
        .draw(display)?;
        Ok(())
    }

    pub fn move_forward(&mut self) {
        let time_moving = (Instant::now() - self.last_time_moved).as_secs_f32();
        self.last_time_moved = Instant::now();
        println!("{}", time_moving);
        self.px += PLAYER_SPEED * time_moving;
    }
}
