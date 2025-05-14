use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, WbDeviceTag,
    WbNodeType_WB_NODE_EMITTER,
};

use crate::Device;

pub struct Emitter(WbDeviceTag);
impl Emitter {
    pub fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_EMITTER, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }
}

impl Device for Emitter {
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
