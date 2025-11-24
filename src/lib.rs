#![no_std]
#![no_main]
#![feature(once_cell_get_mut)]

use core::fmt::Write;
use gba::prelude::*;

pub mod algebra;
pub mod graphics;
pub mod input;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut debug_buffer) = MgbaBufferedLogger::try_new(gba::mgba::MgbaMessageLevel::Fatal) {
        writeln!(debug_buffer, "{info}").ok();
    }
    loop {}
}

