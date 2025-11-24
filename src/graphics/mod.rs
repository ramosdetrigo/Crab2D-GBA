#![allow(dead_code)]
pub mod tilemap;

use crate::algebra::vec2::*;
use gba::prelude::*;

const MEM_VRAM_ADDR: usize = 0x0600_0000;
const VRAM_PAGE_SIZE: usize = 0xA000;

static mut CURRENT_PAGE: *mut u16 = MEM_VRAM_ADDR as *mut u16;

const SCREEN_WIDTH: u32 = 240;
const SCREEN_HEIGHT: u32 = 160;
const M5_SCREEN_WIDTH: u32 = 160;
const M5_SCREEN_HEIGHT: u32 = 128;

#[inline]
/// Flips the back and front pages
pub fn flip_page() {
    unsafe { CURRENT_PAGE = (CURRENT_PAGE as usize ^ VRAM_PAGE_SIZE) as *mut u16 }
    let page_flip = DISPCNT.read().show_frame1();
    DISPCNT.write(DISPCNT.read().with_show_frame1(!page_flip));
}

#[inline]
/// Returns current display mode
pub fn video_mode() -> u32 {
    DISPCNT.read().video_mode() as u32
}

#[inline]
/// Sets full display mode mask
pub fn set_display_mode(mode: DisplayControl) {
    DISPCNT.write(mode)
}

#[inline]
/// Naive vsync function
pub fn vsync() {
    while VCOUNT.read() >= 160 {} // wait till VDraw
    while VCOUNT.read() < 160 {} // wait till VBlank
}

// Bitmap drawing functions
#[inline]
/// Draws a point with specified color at coords (x,y).
pub fn point(p: Vec2<u32>, color: Color) {
    unsafe {
        let screen_width = match video_mode() {
            5 => M5_SCREEN_WIDTH,
            _ => SCREEN_WIDTH,
        };

        CURRENT_PAGE
            .add((p.y * screen_width + p.x) as usize)
            .write(color.0)
    };
}

/// Draws a full rect with specified width + height with top-left at point p and specified color.
pub fn rect(p: Vec2<u32>, width: u32, height: u32, color: Color) {
    for i in 0..=width {
        for j in 0..=height {
            point(p + Vec2::new(i, j), color);
        }
    }
}

/// Draws the frame of a rect with specified width + height with top-left at point p and specified color.
pub fn frame(p: Vec2<u32>, width: u32, height: u32, color: Color) {
    let top_left = p;
    let top_right = p + Vec2::new(width, 0);
    let bottom_left = p + Vec2::new(0, height);
    let bottom_right = p + Vec2::new(width, height);

    line(top_left, top_right, color);
    line(bottom_left, bottom_right, color);
    line(top_left, bottom_left, color);
    line(top_right, bottom_right, color);
}

/// Draws a line from point p1 to point p2 with specified color.
pub fn line(p1: Vec2<u32>, p2: Vec2<u32>, color: Color) {
    let (x1, x2) = (p1.x.min(p2.x), p1.x.max(p2.x));
    let (y1, y2) = (p1.y.min(p2.y), p1.y.max(p2.y));

    let dx: i32 = (x2 - x1) as i32;
    let dy: i32 = (y2 - y1) as i32;

    // Caso 1: linha horizontal
    if dy == 0 {
        for i in 0..=dx as u32 {
            point(Vec2::new(x1 + i, y1), color);
        }
    // Caso 2: linha vertical
    } else if dx == 0 {
        for i in 0..=dy as u32 {
            point(Vec2::new(x1, y1 + i), color);
        }
    }
    // Caso 3: linha diagonal
    else {
        let (mut x, mut y) = (p1.x as i32, p1.y as i32);
        let (x1, y1) = (p2.x as i32, p2.y as i32);

        let (dx, dy) = ((x1 - x).abs(), (y1 - y).abs());
        let (sx, sy) = (((x1 > x) as i32 * 2 - 1), ((y1 > y) as i32 * 2 - 1));

        let mut err = dx - dy;

        loop {
            point(Vec2::new(x as u32, y as u32), color);
            if x == x1 && y == y1 {
                break;
            }

            let e2 = err * 2;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}

