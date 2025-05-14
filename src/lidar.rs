use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_lidar_disable,
    wb_lidar_disable_point_cloud, wb_lidar_enable, wb_lidar_enable_point_cloud, wb_lidar_get_fov,
    wb_lidar_get_frequency, wb_lidar_get_horizontal_resolution, wb_lidar_get_layer_point_cloud,
    wb_lidar_get_layer_range_image, wb_lidar_get_max_frequency, wb_lidar_get_max_range,
    wb_lidar_get_min_frequency, wb_lidar_get_min_range, wb_lidar_get_number_of_layers,
    wb_lidar_get_number_of_points, wb_lidar_get_point_cloud, wb_lidar_get_range_image,
    wb_lidar_get_sampling_period, wb_lidar_get_vertical_fov, wb_lidar_is_point_cloud_enabled,
    wb_lidar_set_frequency, WbDeviceTag, WbLidarPoint, WbNodeType_WB_NODE_LIDAR,
};

use crate::{Device, Sensor};

pub struct Lidar(WbDeviceTag);

impl Lidar {
    pub fn fov(&self) -> f64 {
        unsafe { wb_lidar_get_fov(self.0) }
    }

    pub fn vertical_fov(&self) -> f64 {
        unsafe { wb_lidar_get_vertical_fov(self.0) }
    }

    pub fn max_frequency(&self) -> f64 {
        unsafe { wb_lidar_get_max_frequency(self.0) }
    }

    pub fn min_frequency(&self) -> f64 {
        unsafe { wb_lidar_get_min_frequency(self.0) }
    }

    pub fn frequency(&self) -> f64 {
        unsafe { wb_lidar_get_frequency(self.0) }
    }

    pub fn set_frequency(&self, frequency: f64) {
        unsafe { wb_lidar_set_frequency(self.0, frequency) }
    }

    pub fn max_range(&self) -> f64 {
        unsafe { wb_lidar_get_max_range(self.0) }
    }

    pub fn min_range(&self) -> f64 {
        unsafe { wb_lidar_get_min_range(self.0) }
    }

    pub fn horizontal_resolution(&self) -> i32 {
        unsafe { wb_lidar_get_horizontal_resolution(self.0) }
    }

    pub fn number_of_layers(&self) -> i32 {
        unsafe { wb_lidar_get_number_of_layers(self.0) }
    }

    pub fn range_image(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                wb_lidar_get_range_image(self.0),
                (self.horizontal_resolution() * self.number_of_layers()) as usize,
            )
        }
    }

    pub fn range_image_array(&self) -> Vec<&[f32]> {
        let mut arr = vec![];
        for i in 0..self.number_of_layers() {
            arr.push(self.layer_range_image(i));
        }
        arr
    }

    pub fn layer_range_image(&self, layer: i32) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                wb_lidar_get_layer_range_image(self.0, layer),
                (self.horizontal_resolution()) as usize,
            )
        }
    }

    pub fn number_of_points(&self) -> i32 {
        unsafe { wb_lidar_get_number_of_points(self.0) }
    }

    pub fn disable_point_cloud(&self) {
        unsafe {
            wb_lidar_disable_point_cloud(self.0);
        }
    }

    pub fn enable_point_cloud(&self) {
        unsafe {
            wb_lidar_enable_point_cloud(self.0);
        }
    }

    pub fn is_point_cloud_enabled(&self) -> bool {
        unsafe { wb_lidar_is_point_cloud_enabled(self.0) }.abs() != 0
    }

    pub fn point_cloud(&self) -> &[WbLidarPoint] {
        let number_of_points = self.number_of_points();
        unsafe {
            std::slice::from_raw_parts(wb_lidar_get_point_cloud(self.0), number_of_points as usize)
        }
    }

    pub fn layer_point_cloud(&self, layer: i32) -> &[WbLidarPoint] {
        let number_of_points = self.number_of_points() / self.number_of_layers();
        unsafe {
            std::slice::from_raw_parts(
                wb_lidar_get_layer_point_cloud(self.0, layer),
                number_of_points as usize,
            )
        }
    }
}

impl Device for Lidar {
    fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_LIDAR, unsafe {
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

impl Sensor for Lidar {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_lidar_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_lidar_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_lidar_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
