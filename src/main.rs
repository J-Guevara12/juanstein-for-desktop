//! # JuanStein: A Wolfstein inspired minigame.
//! ## Author: Juan Esteban Guevara Roncancio.

use embedded_graphics::{pixelcolor::Rgb666, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use std::time::SystemTime;
mod map;
mod player;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), std::convert::Infallible> {
    let sys_time = SystemTime::now();
    let mut display: SimulatorDisplay<Rgb666> = SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new().scale(3).max_fps(60).build();
    let mut window = Window::new("Pacman", &output_settings);

    let mut player = player::Player::new(100.0, 180.0);

    'runner: loop {
        let time_before = sys_time.elapsed().unwrap().as_millis();
        map::draw_map(&mut display)?;
        player.draw(&mut display)?;

        window.update(&display);
        let time_now = sys_time.elapsed().unwrap().as_millis();

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'runner Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => {
                    player.update_pos(time_now - time_before, keycode)
                }
                _ => (),
            }
        }
    }
}
