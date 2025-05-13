use webots_bindings::{
    wb_brake_get_motor, wb_brake_get_position_sensor, wb_brake_get_type,
    wb_brake_set_damping_constant, wb_device_get_name, wb_device_get_node_type, WbDeviceTag,
    WbNodeType_WB_NODE_BRAKE,
};

use crate::{Device, JointType, Motor, PositionSensor};

pub struct Brake(WbDeviceTag);

impl Brake {
    pub fn set_damping_constant(&self, damping_constant: f64) {
        unsafe { wb_brake_set_damping_constant(self.0, damping_constant) }
    }

    pub fn type_(&self) -> JointType {
        unsafe { wb_brake_get_type(self.0).into() }
    }

    pub fn motor(&self) -> Motor {
        Motor::new(unsafe { wb_brake_get_motor(self.0) })
    }

    pub fn position_sensor(&self) -> PositionSensor {
        PositionSensor::new(unsafe { wb_brake_get_position_sensor(self.0) })
    }
}

impl Device for Brake {
    fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_BRAKE, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
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
