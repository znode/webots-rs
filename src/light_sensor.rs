use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_light_sensor_disable,
    wb_light_sensor_enable, wb_light_sensor_get_lookup_table,
    wb_light_sensor_get_lookup_table_size, wb_light_sensor_get_sampling_period,
    wb_light_sensor_get_value, WbDeviceTag, WbNodeType_WB_NODE_LIGHT_SENSOR,
};

use crate::{Device, Sensor};

pub struct LightSensor(WbDeviceTag);

impl LightSensor {
    pub fn lookup_table(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                wb_light_sensor_get_lookup_table(self.0),
                wb_light_sensor_get_lookup_table_size(self.0) as usize * 3,
            )
        }
    }

    pub fn value(&self) -> f64 {
        unsafe { wb_light_sensor_get_value(self.0) }
    }
}

impl Device for LightSensor {
    fn new(tag: WbDeviceTag) -> Self {
        assert!({
            let node_type = unsafe { wb_device_get_node_type(tag) };
            node_type == WbNodeType_WB_NODE_LIGHT_SENSOR
        });
        LightSensor(tag)
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

impl Sensor for LightSensor {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_light_sensor_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_light_sensor_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_light_sensor_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
