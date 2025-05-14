use webots_bindings::{
    wb_altimeter_disable, wb_altimeter_enable, wb_altimeter_get_sampling_period,
    wb_altimeter_get_value, wb_device_get_model, wb_device_get_name, wb_device_get_node_type,
    WbDeviceTag, WbNodeType_WB_NODE_ALTIMETER,
};

use crate::{Device, Sensor};

pub struct Altimeter(WbDeviceTag);

impl Altimeter {
    pub fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_ALTIMETER, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }
    pub fn value(&self) -> f64 {
        unsafe { wb_altimeter_get_value(self.0) }
    }
}

impl Device for Altimeter {
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

impl Sensor for Altimeter {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_altimeter_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_altimeter_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_altimeter_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
