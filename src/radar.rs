use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_radar_disable,
    wb_radar_enable, wb_radar_get_horizontal_fov, wb_radar_get_max_range, wb_radar_get_min_range,
    wb_radar_get_number_of_targets, wb_radar_get_sampling_period, wb_radar_get_targets,
    wb_radar_get_vertical_fov, WbDeviceTag, WbNodeType_WB_NODE_RADAR, WbRadarTarget,
};

use crate::{Device, Sensor};

pub struct Radar(WbDeviceTag);

impl Radar {
    pub fn max_range(&self) -> f64 {
        unsafe { wb_radar_get_max_range(self.0) }
    }

    pub fn min_range(&self) -> f64 {
        unsafe { wb_radar_get_min_range(self.0) }
    }

    pub fn horizontal_fov(&self) -> f64 {
        unsafe { wb_radar_get_horizontal_fov(self.0) }
    }

    pub fn vertical_fov(&self) -> f64 {
        unsafe { wb_radar_get_vertical_fov(self.0) }
    }

    pub fn number_of_targets(&self) -> i32 {
        unsafe { wb_radar_get_number_of_targets(self.0) }
    }

    pub fn targets(&self) -> &[WbRadarTarget] {
        unsafe {
            std::slice::from_raw_parts(
                wb_radar_get_targets(self.0),
                self.number_of_targets() as usize * std::mem::size_of::<WbRadarTarget>(),
            )
        }
    }
}

impl Device for Radar {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_RADAR, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
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

impl Sensor for Radar {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_radar_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_radar_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_radar_get_sampling_period(self.0) }
    }
    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
