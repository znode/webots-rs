use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_gps_disable,
    wb_gps_enable, wb_gps_get_coordinate_system, wb_gps_get_sampling_period, wb_gps_get_speed,
    wb_gps_get_speed_vector, wb_gps_get_values, WbDeviceTag, WbNodeType_WB_NODE_GPS,
};

use crate::{Device, Sensor};

pub struct Gps(WbDeviceTag);

impl Gps {
    pub fn coordinate_system(&self) -> u32 {
        unsafe { wb_gps_get_coordinate_system(self.0) }
    }

    pub fn speed(&self) -> f64 {
        unsafe { wb_gps_get_speed(self.0) }
    }

    pub fn speed_vector(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                wb_gps_get_speed_vector(self.0),
                3 * std::mem::size_of::<f64>(),
            )
        }
    }

    pub fn value(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(wb_gps_get_values(self.0), 3 * std::mem::size_of::<f64>())
        }
    }
}

impl Device for Gps {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_GPS, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }

    fn tag(&self) -> WbDeviceTag {
        self.0
    }

    fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr(wb_device_get_name(self.0))
                .to_str()
                .unwrap()
        }
    }

    fn model(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr(wb_device_get_model(self.0))
                .to_str()
                .unwrap()
        }
    }

    fn node_type(&self) -> u32 {
        unsafe { wb_device_get_node_type(self.0) }
    }
}

impl Sensor for Gps {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_gps_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_gps_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_gps_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}

#[allow(unused)]
pub fn to_degrees_minutes_seconds(decimal_degrees: f64) -> String {
    let degrees = decimal_degrees as i32;
    let minutes = ((decimal_degrees - degrees as f64) * 60.0) as i32;
    let seconds = ((((decimal_degrees - degrees as f64) * 60.0) - minutes as f64) * 60.0) as i32;
    format!("{degrees}° {minutes}′ {seconds}″")
}
