use crate::Device;

pub trait Sensor: Device {
    fn enable(&self, sampling_period: i32);

    fn disable(&self);

    fn sampling_period(&self) -> i32;

    fn set_sampling_period(&self, sampling_period: i32);
}
