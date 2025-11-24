#![no_std]
#![no_main]
#![feature(once_cell_get_mut)]

use gba::prelude::*;
use crab2d::graphics::*;
use crab2d::input::*;
use crab2d::algebra::vec2::*;

#[no_mangle]
extern "C" fn main() -> ! {
    #[allow(unused_mut)]
    let mut graphics = DisplayHandler::new();
    let mut input = InputHandler::new();

    graphics.set_display_mode(
        DisplayControl::new()
            .with_show_bg2(true)
            .with_video_mode(VideoMode::_3),
    );

    let mut frame = 0;
    loop {
        graphics.vsync();
        gba::video::video3_clear_to(Color::BLACK);
        input.poll();
        
        if input.key_down(Key::A) {
            graphics.rect(Vec2::new(20, 20), 40, 40, gba::video::Color::YELLOW);
        } else {
            graphics.frame(Vec2::new(20, 20), 40, 40, gba::video::Color::MAGENTA);
        }
        
        frame += 1;
        if frame == 60 {
            frame = 0;
        }
    }
}