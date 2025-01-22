//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

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
    let mut counter: u128;
    counter = 0;
    // Create a new simulator display with 65x65 pixels.
    let mut display: SimulatorDisplay<Rgb666> = SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new().scale(3).max_fps(60).build();
    let mut window = Window::new("Pacman", &output_settings);

    let mut player = player::Player::new(100.0, 200.0);

    'runner: loop {
        if counter != 0 {
            println!(
                "{}",
                1000 / (sys_time.elapsed().unwrap().as_millis() - counter)
            );
        }
        map::draw_map(&mut display)?;
        player.draw(&mut display)?;
        counter = sys_time.elapsed().unwrap().as_millis();

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'runner Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => player.move_forward(),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
