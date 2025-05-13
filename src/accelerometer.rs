use std::slice::from_raw_parts;

use thiserror::Error;
use webots_bindings::{
    wb_accelerometer_disable, wb_accelerometer_enable, wb_accelerometer_get_lookup_table,
    wb_accelerometer_get_lookup_table_size, wb_accelerometer_get_sampling_period,
    wb_accelerometer_get_values, wb_device_get_model, wb_device_get_name, wb_device_get_node_type,
    WbDeviceTag, WbNodeType_WB_NODE_ACCELEROMETER,
};

use crate::{Device, Sensor};

#[derive(Debug, Error)]
pub enum AccelerometerError {
    #[error("failed to get lookup table: lookup table data is NULL")]
    LookupTableIsNull,
    #[error("failed to get values: value data is NULL")]
    ValueIsNull,
}

pub struct Accelerometer(WbDeviceTag);

impl Accelerometer {
    pub fn lookup_table_size(&self) -> i32 {
        unsafe { wb_accelerometer_get_lookup_table_size(self.0) }
    }

    pub fn lookup_table(&self) -> Result<&[f64], AccelerometerError> {
        let lookup_table_size = self.lookup_table_size();
        unsafe {
            let lookup_table = wb_accelerometer_get_lookup_table(self.0);
            if lookup_table.is_null() {
                return Err(AccelerometerError::LookupTableIsNull);
            }
            Ok(from_raw_parts(lookup_table, lookup_table_size as usize))
        }
    }

    pub fn values(&self) -> Result<[f64; 3], AccelerometerError> {
        unsafe {
            let values = wb_accelerometer_get_values(self.0);
            if values.is_null() {
                return Err(AccelerometerError::ValueIsNull);
            }
            Ok([*values.offset(0), *values.offset(1), *values.offset(2)])
        }
    }
}

impl Device for Accelerometer {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_ACCELEROMETER, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
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

impl Sensor for Accelerometer {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_accelerometer_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_accelerometer_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_accelerometer_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, sampling_period: i32) {
        unimplemented!("{sampling_period}")
    }
}
