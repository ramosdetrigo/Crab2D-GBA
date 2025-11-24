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

/// Main interface for handling user inputs (button presses)
pub struct InputHandler {
    input_previous: u16,
    input_current: u16,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            input_previous: 0,
            input_current: 0,
        }
    }

    #[inline]
    /// Update the input handler
    pub fn poll(&mut self) {
        self.input_previous = self.input_current;
        self.input_current = KEYINPUT.read().to_u16();
    }

    #[inline]
    /// True if the specified key is being pressed.
    pub fn key_down(&self, key: Key) -> bool {
        (self.input_current & key as u16) == 0
    }

    #[inline]
    /// True if the specified key is being held down (pressed in both previous and current frame).
    pub fn key_held(&self, key: Key) -> bool {
        self.key_down(key) && ((self.input_previous & key as u16) == 0)
    }

    #[inline]
    /// True if the specified key was just pressed (pressed in current frame, but not previous frame).
    pub fn key_hit(&self, key: Key) -> bool {
        (self.input_current & key as u16 == 0) && (!self.input_previous & key as u16 == 0)
    }

    #[inline]
    /// True if the specified key was just released (not pressed in current frame, but pressed in previous frame).
    pub fn key_released(&self, key: Key) -> bool {
        (self.input_current & key as u16 != 0) && (self.input_previous & key as u16 == 0)
    }
}
