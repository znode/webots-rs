use std::ffi::c_char;

use webots_bindings::{
    wbu_motion_delete, wbu_motion_get_duration, wbu_motion_get_time, wbu_motion_is_over,
    wbu_motion_new, wbu_motion_play, wbu_motion_set_loop, wbu_motion_set_reverse,
    wbu_motion_set_time, wbu_motion_stop, WbMotionRef,
};

pub struct Motion(WbMotionRef);
impl Drop for Motion {
    fn drop(&mut self) {
        unsafe {
            wbu_motion_delete(self.0);
        }
    }
}

impl Motion {
    pub fn new(filename: &str) -> Self {
        Self(unsafe { wbu_motion_new(filename.as_ptr() as *const c_char) })
    }

    pub fn is_valid(&self) -> bool {
        !self.0.is_null()
    }

    pub fn play(&self) {
        unsafe {
            wbu_motion_play(self.0);
        }
    }

    pub fn stop(&self) {
        unsafe {
            wbu_motion_stop(self.0);
        }
    }

    pub fn set_loop(&self, lop: i8) {
        unsafe {
            wbu_motion_set_loop(self.0, lop);
        }
    }

    pub fn set_reverse(&self, reverse: i8) {
        unsafe {
            wbu_motion_set_reverse(self.0, reverse);
        }
    }

    pub fn is_over(&self) -> bool {
        unsafe { wbu_motion_is_over(self.0) != 0 }
    }

    pub fn duration(&self) -> i32 {
        unsafe { wbu_motion_get_duration(self.0) }
    }

    pub fn time(&self) -> i32 {
        unsafe { wbu_motion_get_time(self.0) }
    }

    pub fn set_time(&self, time: i32) {
        unsafe {
            wbu_motion_set_time(self.0, time);
        }
    }
}
