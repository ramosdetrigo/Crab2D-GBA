#![no_std]
#![no_main]
#![feature(once_cell_get_mut)]

use core::fmt::Write;
use gba::prelude::*;

use crab2d::algebra::vec2::*;
use crab2d::graphics;
use crab2d::input::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut debug_buffer) = MgbaBufferedLogger::try_new(gba::mgba::MgbaMessageLevel::Fatal) {
        writeln!(debug_buffer, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
fn main() {
    #[allow(unused_mut)]
    let mut input = InputHandler::new();

    graphics::set_display_mode(
        DisplayControl::new()
            .with_show_bg2(true)
            .with_video_mode(VideoMode::_3),
    );

    let mut frame = 0;
    loop {
        graphics::vsync();
        gba::video::video3_clear_to(Color::BLACK);
        input.poll();

        if input.key_down(Key::A) {
            graphics::rect(Vec2::new(20, 20), 40, 40, gba::video::Color::YELLOW);
        } else {
            graphics::frame(Vec2::new(20, 20), 40, 40, gba::video::Color::MAGENTA);
        }

        frame += 1;
        if frame == 60 {
            frame = 0;
        }
    }
}
