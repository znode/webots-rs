use webots_bindings::WbDeviceTag;

pub trait Device {
    fn new(tag: WbDeviceTag) -> Self;

    fn tag(&self) -> WbDeviceTag;

    fn name(&self) -> &str;

    fn model(&self) -> &str;

    fn node_type(&self) -> u32;
}
