//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

use embedded_graphics::{pixelcolor::Rgb666, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 65x65 pixels.
    let mut display: SimulatorDisplay<Rgb666> = SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut window = Window::new("Pacman", &output_settings);

    loop {
        display.clear(Rgb666::BLACK)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
}
