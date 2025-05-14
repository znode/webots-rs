use std::slice::from_raw_parts;

use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_range_finder_disable,
    wb_range_finder_enable, wb_range_finder_get_fov, wb_range_finder_get_height,
    wb_range_finder_get_max_range, wb_range_finder_get_min_range, wb_range_finder_get_range_image,
    wb_range_finder_get_sampling_period, wb_range_finder_get_width, wb_range_finder_save_image,
    WbDeviceTag, WbNodeType_WB_NODE_RANGE_FINDER,
};

use crate::{Device, Sensor};

pub struct RangeFinder(WbDeviceTag);

impl RangeFinder {
    pub fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_RANGE_FINDER, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }

    pub fn fov(&self) -> f64 {
        unsafe { wb_range_finder_get_fov(self.0) }
    }

    pub fn width(&self) -> i32 {
        unsafe { wb_range_finder_get_width(self.0) }
    }

    pub fn height(&self) -> i32 {
        unsafe { wb_range_finder_get_height(self.0) }
    }

    pub fn min_range(&self) -> f64 {
        unsafe { wb_range_finder_get_min_range(self.0) }
    }

    pub fn max_range(&self) -> f64 {
        unsafe { wb_range_finder_get_max_range(self.0) }
    }

    pub fn range_image(&self) -> &[f32] {
        let width = self.width();
        let height = self.height();
        unsafe {
            from_raw_parts(
                wb_range_finder_get_range_image(self.0),
                (width * height) as usize,
            )
        }
    }

    pub fn range_image_array(&self) -> Vec<&[f32]> {
        let mut arr = vec![];
        let image = self.range_image();
        let width = self.width();
        let height = self.height();

        for i in 0..height {
            arr.push(&image[(i * width) as usize..((i + 1) * width) as usize]);
        }
        arr
    }

    pub fn range_image_get_depth(image: &[f32], width: i32, x: i32, y: i32) -> f32 {
        image[(y * width + x) as usize]
    }

    pub fn save_image(&self, filename: &str, quality: i32) -> i32 {
        unsafe { wb_range_finder_save_image(self.0, filename.as_ptr() as *const i8, quality) }
    }
}

impl Device for RangeFinder {
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

impl Sensor for RangeFinder {
    fn enable(&self, sampling_period: i32) {
        unsafe {
            wb_range_finder_enable(self.0, sampling_period);
        }
    }

    fn disable(&self) {
        unsafe {
            wb_range_finder_disable(self.0);
        }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_range_finder_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}

#[allow(unused)]
pub fn image_get_depth(image: &[f32], width: i32, x: i32, y: i32) -> f32 {
    image[(y * width + x) as usize]
}
