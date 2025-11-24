#![allow(dead_code)]
use gba::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum Key {
    A = 0x0001,
    B = 0x0002,
    SELECT = 0x0004,
    START = 0x0008,
    RIGHT = 0x0010,
    LEFT = 0x0020,
    UP = 0x0040,
    DOWN = 0x0080,
    R = 0x0100,
    L = 0x0200,
}

static mut INPUT_PREVIOUS: u16 = 0;
static mut INPUT_CURRENT: u16 = 0;

#[inline(always)]
pub fn current_input() -> u16 {
    unsafe { INPUT_CURRENT }
}

#[inline(always)]
pub fn previous_input() -> u16 {
    unsafe { INPUT_PREVIOUS }
}

/// Update the input handler
pub fn poll() {
    unsafe {
        INPUT_PREVIOUS = INPUT_CURRENT;
        INPUT_CURRENT = KEYINPUT.read().to_u16();
    }
}

#[inline]
/// True if the specified key is being pressed.
pub fn key_down(key: Key) -> bool {
    (current_input() & key as u16) == 0
}

#[inline]
/// True if the specified key is being held down (pressed in both previous and current frame).
pub fn key_held(key: Key) -> bool {
    key_down(key) && ((previous_input() & key as u16) == 0)
}

#[inline]
/// True if the specified key was just pressed (pressed in current frame, but not previous frame).
pub fn key_hit(key: Key) -> bool {
    (current_input() & key as u16 == 0) && (previous_input() & key as u16 != 0)
}

#[inline]
/// True if the specified key was just released (not pressed in current frame, but pressed in previous frame).
pub fn key_released(key: Key) -> bool {
    (current_input() & key as u16 != 0) && (previous_input() & key as u16 == 0)
}
