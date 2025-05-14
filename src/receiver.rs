use thiserror::Error;
use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_receiver_disable,
    wb_receiver_enable, wb_receiver_get_channel, wb_receiver_get_data, wb_receiver_get_data_size,
    wb_receiver_get_emitter_direction, wb_receiver_get_queue_length,
    wb_receiver_get_sampling_period, wb_receiver_get_signal_strength, wb_receiver_next_packet,
    wb_receiver_set_channel, WbDeviceTag, WbNodeType_WB_NODE_RECEIVER,
};

use crate::{Device, Sensor};

#[derive(Debug, Error)]
pub enum ReceiverError {
    #[error("failed to get emitter direction: emitter direction data is NULL")]
    EmitterDirectionIsNull,
}

pub struct Receiver(WbDeviceTag);

impl Receiver {
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                wb_receiver_get_data(self.0) as *const u8,
                self.data_size() as usize,
            )
        }
    }

    pub fn data_size(&self) -> i32 {
        unsafe { wb_receiver_get_data_size(self.0) }
    }

    pub fn floats(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                wb_receiver_get_data(self.0) as *const f64,
                self.data_size() as usize / std::mem::size_of::<f64>(),
            )
        }
    }

    pub fn ints(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                wb_receiver_get_data(self.0) as *const i32,
                self.data_size() as usize / std::mem::size_of::<i32>(),
            )
        }
    }

    pub fn bools(&self) -> Vec<bool> {
        self.bytes().iter().map(|&x| x != 0).collect()
    }

    pub fn queue_length(&self) -> i32 {
        unsafe { wb_receiver_get_queue_length(self.0) }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                wb_receiver_get_data(self.0) as *const u8,
                self.data_size() as usize,
            ))
        }
    }

    pub fn next_package(&self) {
        unsafe { wb_receiver_next_packet(self.0) };
    }

    pub fn signal_strength(&self) -> f64 {
        unsafe { wb_receiver_get_signal_strength(self.0) }
    }

    pub fn emitter_direction(&self) -> &[f64] {
        unsafe {
            std::slice::from_raw_parts(
                wb_receiver_get_emitter_direction(self.0),
                3 * std::mem::size_of::<f64>(),
            )
        }
    }

    pub fn next(&self) -> Result<Option<Packet>, ReceiverError> {
        let queue_length = unsafe { wb_receiver_get_queue_length(self.0) };
        if queue_length > 0 {
            let data = self.bytes().to_vec();
            let signal_strength = self.signal_strength();
            let emitter_direction = self.emitter_direction();

            let packet = Packet {
                data,
                signal_strength,
                emitter_direction: [
                    emitter_direction[0],
                    emitter_direction[1],
                    emitter_direction[2],
                ],
            };
            self.next_package();

            Ok(Some(packet))
        } else {
            Ok(None)
        }
    }

    pub fn set_channel(&self, channel: i32) {
        unsafe { wb_receiver_set_channel(self.0, channel) }
    }

    pub fn channel(&self) -> i32 {
        unsafe { wb_receiver_get_channel(self.0) }
    }
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub data: Vec<u8>,
    pub signal_strength: f64,
    pub emitter_direction: [f64; 3],
}

impl Device for Receiver {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_RECEIVER, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }

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

impl Sensor for Receiver {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_receiver_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_receiver_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_receiver_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
