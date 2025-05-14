use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_position_sensor_disable,
    wb_position_sensor_enable, wb_position_sensor_get_brake, wb_position_sensor_get_motor,
    wb_position_sensor_get_sampling_period, wb_position_sensor_get_type,
    wb_position_sensor_get_value, WbDeviceTag, WbNodeType_WB_NODE_POSITION_SENSOR,
};

use crate::{Brake, Device, JointType, Motor, Sensor};

pub struct PositionSensor(WbDeviceTag);

impl PositionSensor {
    pub fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_POSITION_SENSOR, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }
    pub fn value(&self) -> f64 {
        unsafe { wb_position_sensor_get_value(self.0) }
    }

    pub fn type_(&self) -> JointType {
        unsafe { wb_position_sensor_get_type(self.0).into() }
    }

    pub fn motor(&self) -> Motor {
        Motor::new(unsafe { wb_position_sensor_get_motor(self.0) })
    }

    pub fn brake(&self) -> Brake {
        Brake::new(unsafe { wb_position_sensor_get_brake(self.0) })
    }
}

impl Device for PositionSensor {
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

impl Sensor for PositionSensor {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_position_sensor_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_position_sensor_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_position_sensor_get_sampling_period(self.0) }
    }
    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
