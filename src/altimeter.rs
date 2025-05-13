use webots_bindings::{
    wb_altimeter_get_value, wb_device_get_name, wb_device_get_node_type, WbDeviceTag,
    WbNodeType_WB_NODE_ALTIMETER,
};

use crate::{Device, Sensor};

pub struct Altimeter(WbDeviceTag);

impl Altimeter {
    pub fn value(&self) -> f64 {
        unsafe { wb_altimeter_get_value(self.0) }
    }
}

impl Device for Altimeter {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_ALTIMETER, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }
    fn name(&self) -> &str {
        unsafe {
            let name = wb_device_get_name(self.0);
            crate::utils::cstr_to_str(name).unwrap_or("Unknown")
        }
    }

    fn model(&self) -> &str {
        todo!()
    }

    fn node_type(&self) -> u32 {
        todo!()
    }
}

impl Sensor for Altimeter {
    fn enable(&self, sampling_period: i32) {
        todo!()
    }

    fn disable(&self) {
        todo!()
    }

    fn sampling_period(&self) -> i32 {
        todo!()
    }

    fn set_sampling_period(&self, sampling_period: i32) {
        todo!()
    }
}
