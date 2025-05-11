use std::slice::from_raw_parts;

use thiserror::Error;

use webots_bindings::{
    wb_device_get_node_type, wb_range_finder_disable, wb_range_finder_enable,
    wb_range_finder_get_fov, wb_range_finder_get_height, wb_range_finder_get_max_range,
    wb_range_finder_get_min_range, wb_range_finder_get_range_image,
    wb_range_finder_get_sampling_period, wb_range_finder_get_width, wb_range_finder_save_image,
    WbDeviceTag, WbNodeType_WB_NODE_RANGE_FINDER,
};

#[derive(Debug, Error)]
pub enum RangeFinerError {
    #[error("failed to get image: image data is NULL")]
    DataIsNull,
}

pub struct RangeFinder(WbDeviceTag);

impl RangeFinder {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_RANGE_FINDER, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe {
            wb_range_finder_enable(self.0, sampling_period);
        }
    }

    pub fn disable(&self) {
        unsafe {
            wb_range_finder_disable(self.0);
        }
    }

    pub fn sample_period(&self) -> i32 {
        unsafe { wb_range_finder_get_sampling_period(self.0) }
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

    pub fn range_image(&self) -> Result<&[f32], RangeFinerError> {
        let width = self.width();
        let height = self.height();
        unsafe {
            let image = wb_range_finder_get_range_image(self.0);
            if image.is_null() {
                return Err(RangeFinerError::DataIsNull);
            }
            Ok(from_raw_parts(image, (width * height) as usize))
        }
    }

    pub fn save_image(&self, filename: &str, quality: i32) -> i32 {
        unsafe { wb_range_finder_save_image(self.0, filename.as_ptr() as *const i8, quality) }
    }
}

#[allow(unused)]
pub fn image_get_depth(image: &[f32], width: i32, x: i32, y: i32) -> f32 {
    image[(y * width + x) as usize]
}
