#![no_std]
#![no_main]
use core::fmt::Write;
use gba::prelude::*;

use micromath::vector::F32x2;
use micromath::vector::Vector2d;
#[allow(unused_imports)]
use micromath::F32Ext;

use crab2d::algebra::vec2::*;
use crab2d::graphics;
use crab2d::input;
use crab2d::input::Key;

// ----- Game elements -----
const PADDLE_WIDTH: u32 = 6;
const PADDLE_HEIGHT: u32 = 30;
const BALL_SIZE: u32 = 6;

const PADDLE_SPEED: i32 = 2;

struct Paddle {
    pub pos: Vec2<i32>,
}

impl Paddle {
    pub fn new(pos: Vec2<i32>) -> Self {
        Self { pos }
    }

    pub fn draw(&self) {
        graphics::rect(
            Vec2::new(self.pos.x as u32, self.pos.y as u32),
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            Color::WHITE,
        );
    }

    pub fn follow_input(&mut self) {
        if input::key_down(Key::UP) {
            self.pos.y -= PADDLE_SPEED;
            self._clamp_pos();
        }
        if input::key_down(Key::DOWN) {
            self.pos.y += PADDLE_SPEED;
            self._clamp_pos();
        }
    }

    pub fn follow_ball(&mut self, ball_y: i32) {
        let center = self.pos.y + PADDLE_HEIGHT as i32 / 2;
        let dir = ball_y - center;
        self.pos.y += dir.clamp(-PADDLE_SPEED, PADDLE_SPEED);
        self._clamp_pos();
    }

    fn _clamp_pos(&mut self) {
        self.pos.y = self.pos.y.clamp(0, 160 - PADDLE_HEIGHT as i32);
    }
}

struct Ball {
    pub pos: Vec2<i32>,
    pub speed: F32x2,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: Vec2 {
                x: 240 / 2 - BALL_SIZE as i32 / 2,
                y: 160 / 2 - BALL_SIZE as i32 / 2,
            },
            speed: Vector2d { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self) {
        if self.pos.y <= 0 || self.pos.y + BALL_SIZE as i32 > 240 {
            self.speed.y = -self.speed.y;
        }
        let dx = self.speed.x as i32;
        let dy = self.speed.y as i32;

        self.pos.x += dx;
        self.pos.y += dy;
    }

    pub fn draw(&self) {
        graphics::rect(
            Vec2::new(self.pos.x as u32, self.pos.y as u32),
            BALL_SIZE,
            BALL_SIZE,
            Color::WHITE,
        );
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut debug_buffer) = MgbaBufferedLogger::try_new(gba::mgba::MgbaMessageLevel::Fatal) {
        writeln!(debug_buffer, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
fn main() {
    graphics::set_display_mode(
        DisplayControl::new()
            .with_show_bg2(true)
            .with_video_mode(VideoMode::_3),
    );

    let mut player = Paddle::new(Vec2::new(36, 80 - PADDLE_HEIGHT as i32 / 2));
    let mut bot = Paddle::new(Vec2::new(
        240 - 36 - PADDLE_WIDTH as i32,
        80 - PADDLE_HEIGHT as i32 / 2,
    ));
    let mut ball = Ball::new();

    loop {
        input::poll();
        gba::video::video3_clear_to(Color::from_rgb(6, 6, 10));

        // Update
        ball.update();
        player.follow_input();
        bot.follow_ball(30);

        // Draw
        player.draw();
        bot.draw();
        ball.draw();
        graphics::line(Vec2::new(120, 0), Vec2::new(120, 160), Color::WHITE); // Middle line
        
        graphics::vsync();
    }
}
