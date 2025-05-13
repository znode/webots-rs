use webots_bindings::{
    wb_device_get_node_type, wb_led_get, wb_led_set, WbDeviceTag, WbNodeType_WB_NODE_LED,
};

use crate::Device;

pub struct Led(WbDeviceTag);

impl Led {
    pub fn set(&self, v: i32) {
        unsafe { wb_led_set(self.0, v) }
    }

    pub fn get(&self) -> i32 {
        unsafe { wb_led_get(self.0) }
    }
}

impl Device for Led {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_LED, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn model(&self) -> &str {
        todo!()
    }

    fn node_type(&self) -> u32 {
        todo!()
    }
}
