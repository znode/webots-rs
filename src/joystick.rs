use webots_bindings::{
    wb_joystick_disable, wb_joystick_enable, wb_joystick_get_axis_value, wb_joystick_get_model,
    wb_joystick_get_number_of_axes, wb_joystick_get_number_of_povs, wb_joystick_get_pov_value,
    wb_joystick_get_pressed_button, wb_joystick_get_sampling_period, wb_joystick_is_connected,
    wb_joystick_set_auto_centering_gain, wb_joystick_set_constant_force,
    wb_joystick_set_constant_force_duration, wb_joystick_set_force_axis,
    wb_joystick_set_resistance_gain,
};

use crate::utils::cstr_to_str;

#[derive(Default)]
pub struct Joystick;

impl Joystick {
    pub fn enable(&self, sampling_period: i32) {
        unsafe {
            wb_joystick_enable(sampling_period);
        }
    }

    pub fn disable(&self) {
        unsafe {
            wb_joystick_disable();
        }
    }

    pub fn sampling_period(&self) -> i32 {
        unsafe { wb_joystick_get_sampling_period() }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { wb_joystick_is_connected() != 0 }
    }

    pub fn number_of_axes(&self) -> i32 {
        unsafe { wb_joystick_get_number_of_axes() }
    }

    pub fn axis_value(&self, axis: i32) -> i32 {
        unsafe { wb_joystick_get_axis_value(axis) }
    }

    pub fn number_of_povs(&self) -> i32 {
        unsafe { wb_joystick_get_number_of_povs() }
    }

    pub fn pov_value(&self, pov: i32) -> i32 {
        unsafe { wb_joystick_get_pov_value(pov) }
    }

    pub fn pressed_button(&self) -> i32 {
        unsafe { wb_joystick_get_pressed_button() }
    }

    pub fn set_constant_force(&self, level: i32) {
        unsafe {
            wb_joystick_set_constant_force(level);
        }
    }

    pub fn set_constant_force_duration(&self, duration: f64) {
        unsafe {
            wb_joystick_set_constant_force_duration(duration);
        }
    }

    pub fn set_auto_centerin_gain(&self, gain: f64) {
        unsafe {
            wb_joystick_set_auto_centering_gain(gain);
        }
    }

    pub fn set_resistance_gain(&self, gain: f64) {
        unsafe {
            wb_joystick_set_resistance_gain(gain);
        }
    }

    pub fn set_force_axis(&self, axis: i32) {
        unsafe {
            wb_joystick_set_force_axis(axis);
        }
    }

    pub fn model(&self) -> &str {
        unsafe {
            let m = wb_joystick_get_model();
            cstr_to_str(m).unwrap_or("default")
        }
    }
}
