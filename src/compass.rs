use webots_bindings::WbDeviceTag;

use crate::{Device, Sensor};

pub struct Compass(WbDeviceTag);

impl Compass {
    //
}

impl Device for Compass {
    fn new(tag: WbDeviceTag) -> Self {
        todo!()
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

impl Sensor for Compass {
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
