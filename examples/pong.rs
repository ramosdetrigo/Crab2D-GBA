#![no_std]
#![no_main]
use micromath::F32Ext;
use micromath::vector::F32x2;

use core::fmt::Write;
use gba::prelude::*;

use crab2d::algebra::vec2::*;
use crab2d::graphics::*;
use crab2d::input::*;

struct Paddle {
    pub pos: Vec2<u32>,
    controllable: bool
}



struct Ball {
    pub pos: Vec2<u32>,
    pub speed: F32x2,
}

