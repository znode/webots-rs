use std::fmt::Display;

use webots_bindings::{
    wb_keyboard_disable, wb_keyboard_enable, wb_keyboard_get_key, wb_keyboard_get_sampling_period,
    WB_KEYBOARD_ALT, WB_KEYBOARD_CONTROL, WB_KEYBOARD_DOWN, WB_KEYBOARD_END, WB_KEYBOARD_HOME,
    WB_KEYBOARD_KEY, WB_KEYBOARD_LEFT, WB_KEYBOARD_NUMPAD_DOWN, WB_KEYBOARD_NUMPAD_END,
    WB_KEYBOARD_NUMPAD_HOME, WB_KEYBOARD_NUMPAD_LEFT, WB_KEYBOARD_NUMPAD_RIGHT,
    WB_KEYBOARD_NUMPAD_UP, WB_KEYBOARD_PAGEDOWN, WB_KEYBOARD_PAGEUP, WB_KEYBOARD_RIGHT,
    WB_KEYBOARD_SHIFT, WB_KEYBOARD_UP,
};

pub struct Keyboard;
impl Display for Keyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(k) = self.key() {
            match k {
                WB_KEYBOARD_SHIFT => s += "shift-",
                WB_KEYBOARD_CONTROL => s += "control-",
                WB_KEYBOARD_ALT => s += "alt-",
                _ => match k & WB_KEYBOARD_KEY {
                    WB_KEYBOARD_END => s += "end",
                    WB_KEYBOARD_HOME => s += "home",
                    WB_KEYBOARD_LEFT => s += "left",
                    WB_KEYBOARD_RIGHT => s += "right",
                    WB_KEYBOARD_UP => s += "up",
                    WB_KEYBOARD_DOWN => s += "down",
                    WB_KEYBOARD_PAGEUP => s += "page up",
                    WB_KEYBOARD_PAGEDOWN => s += "page down",
                    WB_KEYBOARD_NUMPAD_END => s += "numpad end",
                    WB_KEYBOARD_NUMPAD_HOME => s += "numpad home",
                    WB_KEYBOARD_NUMPAD_LEFT => s += "numpad left",
                    WB_KEYBOARD_NUMPAD_RIGHT => s += "numpad right",
                    WB_KEYBOARD_NUMPAD_UP => s += "numpad up",
                    WB_KEYBOARD_NUMPAD_DOWN => s += "numpad down",
                    _ => {}
                },
            }
        }
        write!(f, "{}", s)
    }
}

impl Keyboard {
    pub const END: u32 = WB_KEYBOARD_END;
    pub const HOME: u32 = WB_KEYBOARD_HOME;
    pub const LEFT: u32 = WB_KEYBOARD_LEFT;
    pub const UP: u32 = WB_KEYBOARD_UP;
    pub const RIGHT: u32 = WB_KEYBOARD_RIGHT;
    pub const DOWN: u32 = WB_KEYBOARD_DOWN;
    pub const PAGEUP: u32 = WB_KEYBOARD_PAGEUP;
    pub const PAGEDOWN: u32 = WB_KEYBOARD_PAGEDOWN;
    pub const NUMPAD_HOME: u32 = WB_KEYBOARD_NUMPAD_HOME;
    pub const NUMPAD_LEFT: u32 = WB_KEYBOARD_NUMPAD_LEFT;
    pub const NUMPAD_UP: u32 = WB_KEYBOARD_NUMPAD_UP;
    pub const NUMPAD_RIGHT: u32 = WB_KEYBOARD_NUMPAD_RIGHT;
    pub const NUMPAD_DOWN: u32 = WB_KEYBOARD_NUMPAD_DOWN;
    pub const NUMPAD_END: u32 = WB_KEYBOARD_NUMPAD_END;
    pub const KEY: u32 = WB_KEYBOARD_KEY;
    pub const SHIFT: u32 = WB_KEYBOARD_SHIFT;
    pub const CONTROL: u32 = WB_KEYBOARD_CONTROL;
    pub const ALT: u32 = WB_KEYBOARD_ALT;

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_keyboard_enable(sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_keyboard_disable() }
    }

    pub fn sampling_period(&self) -> i32 {
        unsafe { wb_keyboard_get_sampling_period() }
    }

    pub fn key(&self) -> Option<u32> {
        unsafe {
            let key = wb_keyboard_get_key();
            if key == -1 {
                None
            } else {
                Some(key as u32)
            }
        }
    }
}
