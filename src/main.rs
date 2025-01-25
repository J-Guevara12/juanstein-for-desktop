//! # JuanStein: A Wolfstein inspired minigame.
//! ## Author: Juan Esteban Guevara Roncancio.

use embedded_graphics::{pixelcolor::Rgb666, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use std::time::SystemTime;
mod map;
mod player;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), std::convert::Infallible> {
    let sys_time = SystemTime::now();
    let mut display: SimulatorDisplay<Rgb666> = SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new().scale(3).max_fps(600).build();
    let mut window = Window::new("JuanStein for desktop", &output_settings);

    let mut player = player::Player::new(1.5, 1.5);
    let (mut delta, mut time_now, mut rendering, mut updating, mut events);
    time_now = sys_time.elapsed().unwrap().as_millis();

    'runner: loop {
        delta = sys_time.elapsed().unwrap().as_millis() - time_now;
        time_now = sys_time.elapsed().unwrap().as_millis();

        map::draw_map(&mut display)?;
        player.draw(&mut display)?;

        rendering = sys_time.elapsed().unwrap().as_millis() - time_now;

        window.update(&display);
        updating = sys_time.elapsed().unwrap().as_millis() - time_now - rendering;

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'runner Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => player.update_pos(delta, keycode),
                _ => (),
            }
        }
        events = sys_time.elapsed().unwrap().as_millis() - time_now - rendering - updating;
        println!("{}: {}, {}, {}", delta, rendering, updating, events);
    }
}
