use std::slice::from_raw_parts;

use thiserror::Error;
use webots_bindings::{
    wb_device_get_name, wb_device_get_node_type, wb_distance_sensor_disable,
    wb_distance_sensor_enable, wb_distance_sensor_get_aperture,
    wb_distance_sensor_get_lookup_table, wb_distance_sensor_get_lookup_table_size,
    wb_distance_sensor_get_max_value, wb_distance_sensor_get_min_value,
    wb_distance_sensor_get_sampling_period, wb_distance_sensor_get_type,
    wb_distance_sensor_get_value, WbDeviceTag, WbNodeType_WB_NODE_DISTANCE_SENSOR,
};

use crate::{Device, DistanceSensorType, Sensor};

#[derive(Debug, Error)]
pub enum DistanceSensorError {
    #[error("failed to get lookup table: lookup table data is NULL")]
    LookupTableIsNull,
}

pub struct DistanceSensor(WbDeviceTag);

impl DistanceSensor {
    pub fn value(&self) -> f64 {
        unsafe { wb_distance_sensor_get_value(self.0) }
    }

    pub fn max_value(&self) -> f64 {
        unsafe { wb_distance_sensor_get_max_value(self.0) }
    }

    pub fn min_value(&self) -> f64 {
        unsafe { wb_distance_sensor_get_min_value(self.0) }
    }

    pub fn aperture(&self) -> f64 {
        unsafe { wb_distance_sensor_get_aperture(self.0) }
    }

    pub fn lookup_table_size(&self) -> i32 {
        unsafe { wb_distance_sensor_get_lookup_table_size(self.0) }
    }

    pub fn lookup_table(&self) -> Result<&[f64], DistanceSensorError> {
        let lookup_table_size = self.lookup_table_size();
        unsafe {
            let lookup_table = wb_distance_sensor_get_lookup_table(self.0);
            if lookup_table.is_null() {
                return Err(DistanceSensorError::LookupTableIsNull);
            }
            Ok(from_raw_parts(lookup_table, lookup_table_size as usize))
        }
    }

    pub fn type_(&self) -> DistanceSensorType {
        unsafe { wb_distance_sensor_get_type(self.0).into() }
    }
}

impl Device for DistanceSensor {
    fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_DISTANCE_SENSOR, unsafe {
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

impl Sensor for DistanceSensor {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_distance_sensor_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_distance_sensor_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_distance_sensor_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, sampling_period: i32) {
        todo!()
    }
}
