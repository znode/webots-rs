use webots_bindings::{
    wb_compass_disable, wb_compass_enable, wb_compass_get_lookup_table,
    wb_compass_get_lookup_table_size, wb_compass_get_sampling_period, wb_compass_get_values,
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, WbDeviceTag,
    WbNodeType_WB_NODE_COMPASS,
};

use crate::{Device, Sensor};

pub struct Compass(WbDeviceTag);

impl Compass {
    pub fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_COMPASS, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }
    pub fn values(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(wb_compass_get_values(self.0), 3) }
    }

    pub fn lookup_table(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                wb_compass_get_lookup_table(self.0),
                wb_compass_get_lookup_table_size(self.0) as usize * 3,
            )
        }
    }
}

impl Device for Compass {
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

impl Sensor for Compass {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_compass_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_compass_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_compass_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
