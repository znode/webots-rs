use thiserror::Error;
use webots_bindings::{
    wb_device_get_name, wb_device_get_node_type, wb_inertial_unit_disable, wb_inertial_unit_enable,
    wb_inertial_unit_get_noise, wb_inertial_unit_get_quaternion,
    wb_inertial_unit_get_roll_pitch_yaw, wb_inertial_unit_get_sampling_period, WbDeviceTag,
    WbNodeType_WB_NODE_INERTIAL_UNIT,
};

use crate::{Device, Sensor};

#[derive(Debug, Error)]
pub enum InertialUnitError {
    #[error("failed to get roll/pitch/yaw: roll/pitch/yaw data is NULL")]
    RollPitchYawDataIsNull,
    #[error("failed to get quaternion: quaternion data is NULL")]
    QuaternionDataIsNull,
}

pub struct InertialUnit(WbDeviceTag);

impl InertialUnit {
    pub fn noise(&self) -> f64 {
        unsafe { wb_inertial_unit_get_noise(self.0) }
    }

    pub fn roll_pitch_yaw(&self) -> Result<[f64; 3], InertialUnitError> {
        unsafe {
            let roll_pitch_yaw = wb_inertial_unit_get_roll_pitch_yaw(self.0);
            if roll_pitch_yaw.is_null() {
                return Err(InertialUnitError::RollPitchYawDataIsNull);
            }
            Ok([
                *roll_pitch_yaw.offset(0),
                *roll_pitch_yaw.offset(1),
                *roll_pitch_yaw.offset(2),
            ])
        }
    }

    pub fn quaternion(&self) -> Result<[f64; 4], InertialUnitError> {
        unsafe {
            let quaternion = wb_inertial_unit_get_quaternion(self.0);
            if quaternion.is_null() {
                return Err(InertialUnitError::QuaternionDataIsNull);
            }
            Ok([
                *quaternion.offset(0),
                *quaternion.offset(1),
                *quaternion.offset(2),
                *quaternion.offset(3),
            ])
        }
    }
}

impl Device for InertialUnit {
    fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_INERTIAL_UNIT, unsafe {
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

impl Sensor for InertialUnit {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_inertial_unit_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_inertial_unit_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_inertial_unit_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, sampling_period: i32) {
        todo!()
    }
}
