use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder, Rectangle},
};

use embedded_graphics_simulator::sdl2::Keycode;

const PLAYER_SIZE: u32 = 3;
const PLAYER_SPEED: f32 = 24.0;
const PLAYER_COLOR: Rgb666 = Rgb666::CSS_AQUA;
const LINE_COLOR: Rgb666 = Rgb666::CSS_INDIAN_RED;
const PI: f32 = 3.14159265359;

pub struct Player {
    px: f32,
    py: f32,
    theta: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        return Player {
            px: x,
            py: y,
            theta: 0.0,
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

        let (dx, dy) = (5.0 * libm::cosf(self.theta), 5.0 * libm::sinf(self.theta));

        Line::new(
            Point::new(self.px as i32, self.py as i32),
            Point::new((self.px + dx) as i32, (self.py + dy) as i32),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(LINE_COLOR)
                .stroke_width(1)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }

    fn move_player(&mut self, displacement: f32, theta: f32) {
        self.px += displacement * libm::cosf(theta);
        self.py += displacement * libm::sinf(theta);
    }

    pub fn update_pos(&mut self, time_moving: u128, keycode: Keycode) {
        let displacement = time_moving as f32 * PLAYER_SPEED / 1000.0;
        match keycode {
            Keycode::W => self.move_player(displacement, self.theta),
            Keycode::A => self.move_player(displacement, self.theta + PI / 2.0),
            Keycode::S => self.move_player(displacement, self.theta + PI),
            Keycode::D => self.move_player(displacement, self.theta - PI / 2.0),
            _ => (),
        }
    }
}
