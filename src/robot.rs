use std::ffi::{CStr, CString};

use webots_bindings::{
    wb_robot_battery_sensor_disable, wb_robot_battery_sensor_enable,
    wb_robot_battery_sensor_get_sampling_period, wb_robot_battery_sensor_get_value,
    wb_robot_cleanup, wb_robot_get_basic_time_step, wb_robot_get_custom_data, wb_robot_get_device,
    wb_robot_get_mode, wb_robot_get_model, wb_robot_get_name, wb_robot_get_project_path,
    wb_robot_get_supervisor, wb_robot_get_synchronization, wb_robot_get_time, wb_robot_get_urdf,
    wb_robot_get_world_path, wb_robot_init, wb_robot_set_custom_data, wb_robot_set_mode,
    wb_robot_step,
};

use crate::{
    Accelerometer, Altimeter, Brake, Camera, Compass, Display, DistanceSensor, Emitter, Gps, Gyro,
    InertialUnit, Joystick, Keyboard, Led, Lidar, LightSensor, Motor, Mouse, PositionSensor, Radar,
    RangeFinder, Receiver, RobotMode, TouchSensor,
};

pub struct Robot;

impl Drop for Robot {
    fn drop(&mut self) {
        unsafe {
            wb_robot_cleanup();
        }
    }
}

impl Robot {
    pub fn init() {
        unsafe {
            wb_robot_init();
        }
    }

    pub fn step(duration: i32) -> i32 {
        unsafe { wb_robot_step(duration) }
    }

    pub fn time() -> f64 {
        unsafe { wb_robot_get_time() }
    }

    pub fn urdf(prefix: &str) -> &[u8] {
        let prefix = CString::new(prefix).expect("CString::new failed");
        unsafe {
            let urdf = wb_robot_get_urdf(prefix.as_ptr());
            CStr::from_ptr(urdf).to_bytes()
        }
    }

    pub fn name<'a>() -> &'a str {
        unsafe {
            let name = wb_robot_get_name();
            CStr::from_ptr(name).to_str().unwrap()
        }
    }

    pub fn model<'a>() -> &'a str {
        unsafe {
            let model = wb_robot_get_model();
            CStr::from_ptr(model).to_str().unwrap()
        }
    }

    pub fn custom_data<'a>() -> &'a [u8] {
        unsafe {
            let custom_data = wb_robot_get_custom_data();
            CStr::from_ptr(custom_data).to_bytes()
        }
    }

    pub fn set_custom_data(custom_data: &[u8]) {
        let custom_data =
            CStr::from_bytes_with_nul(custom_data).expect("CStr::from_bytes_with_nul failed");
        unsafe { wb_robot_set_custom_data(custom_data.as_ptr()) }
    }

    pub fn mode() -> RobotMode {
        unsafe { wb_robot_get_mode().into() }
    }

    pub fn set_mode(mode: RobotMode, argument: &str) {
        let argument = CString::new(argument).expect("CString::new failed");
        unsafe { wb_robot_set_mode(mode.into(), argument.as_ptr()) }
    }

    pub fn synchronization() -> bool {
        unsafe { wb_robot_get_synchronization() != 0 }
    }

    pub fn supervisor() -> bool {
        unsafe { wb_robot_get_supervisor() != 0 }
    }

    pub fn project_path<'a>() -> &'a str {
        unsafe {
            let project_path = wb_robot_get_project_path();
            CStr::from_ptr(project_path).to_str().expect("CStr::to_str")
        }
    }

    pub fn world_path<'a>() -> &'a str {
        unsafe {
            let world_path = wb_robot_get_world_path();
            CStr::from_ptr(world_path).to_str().expect("CStr::to_str")
        }
    }

    pub fn basic_time_step() -> f64 {
        unsafe { wb_robot_get_basic_time_step() }
    }

    pub fn battery_sensor_enable(sampling_period: i32) {
        unsafe { wb_robot_battery_sensor_enable(sampling_period) }
    }

    pub fn battery_sensor_disable() {
        unsafe { wb_robot_battery_sensor_disable() }
    }

    pub fn battery_sensor_get_sampling_period() -> i32 {
        unsafe { wb_robot_battery_sensor_get_sampling_period() }
    }

    pub fn battery_sensor_get_value() -> f64 {
        unsafe { wb_robot_battery_sensor_get_value() }
    }

    pub fn accelerometer(name: &str) -> Accelerometer {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Accelerometer::new(device)
    }

    pub fn altimeter(name: &str) -> Altimeter {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Altimeter::new(device)
    }

    pub fn brake(name: &str) -> Brake {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Brake::new(device)
    }

    pub fn camera(name: &str) -> Camera {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Camera::new(device)
    }

    pub fn compass(name: &str) -> Compass {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Compass::new(device)
    }

    pub fn display(name: &str) -> Display {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Display::new(device)
    }

    pub fn distance_sensor(name: &str) -> DistanceSensor {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        DistanceSensor::new(device)
    }

    pub fn emitter(name: &str) -> Emitter {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Emitter::new(device)
    }

    pub fn gps(name: &str) -> Gps {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Gps::new(device)
    }

    pub fn gyro(name: &str) -> Gyro {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Gyro::new(device)
    }

    pub fn inertial_unit(name: &str) -> InertialUnit {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        InertialUnit::new(device)
    }

    pub fn joystick() -> Joystick {
        Joystick
    }

    pub fn keyboard() -> Keyboard {
        Keyboard
    }

    pub fn led(name: &str) -> Led {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Led::new(device)
    }

    pub fn lidar(name: &str) -> Lidar {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Lidar::new(device)
    }

    pub fn light_sensor(name: &str) -> LightSensor {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        LightSensor::new(device)
    }

    pub fn motor(name: &str) -> Motor {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Motor::new(device)
    }

    pub fn mouse() -> Mouse {
        Mouse
    }

    pub fn position_sensor(name: &str) -> PositionSensor {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        PositionSensor::new(device)
    }

    pub fn radar(name: &str) -> Radar {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Radar::new(device)
    }

    pub fn range_finder(name: &str) -> RangeFinder {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        RangeFinder::new(device)
    }

    pub fn receiver(name: &str) -> Receiver {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        Receiver::new(device)
    }

    pub fn touch_sensor(name: &str) -> TouchSensor {
        let name = CString::new(name).expect("CString::new failed");
        let device = unsafe { wb_robot_get_device(name.as_ptr()) };
        TouchSensor::new(device)
    }
}
