use std::{
    ffi::{CStr, CString},
    slice::from_raw_parts,
};

use thiserror::Error;
use webots_bindings::{
    wb_camera_recognition_disable, wb_camera_recognition_disable_segmentation,
    wb_camera_recognition_enable, wb_camera_recognition_enable_segmentation,
    wb_camera_recognition_get_number_of_objects, wb_camera_recognition_get_objects,
    wb_camera_recognition_get_sampling_period, wb_camera_recognition_get_segmentation_image,
    wb_camera_recognition_has_segmentation, wb_camera_recognition_is_segmentation_enabled,
    wb_camera_recognition_save_segmentation_image, wb_device_get_model, wb_device_get_name,
    wb_device_get_node_type, WbDeviceTag, WbNodeType_WB_NODE_CAMERA,
};

use crate::{Camera, Device, Sensor};

#[derive(Debug, Error)]
pub enum RecognitionError {
    #[error("failed to get objects: objects data is NULL")]
    ObjectsDataIsNull,
}

pub struct RecognitionObject<'a> {
    pub id: i32,
    pub position: [f64; 3],
    pub orientation: [f64; 4],
    pub size: [f64; 2],
    pub position_on_image: [i32; 2],
    pub size_on_image: [i32; 2],
    pub number_of_colors: i32,
    pub colors: &'a [f64],
    pub model: &'a str,
}

pub struct Recognition(WbDeviceTag);

impl Recognition {
    pub fn new(camera_device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_CAMERA, unsafe {
            wb_device_get_node_type(camera_device)
        });
        Self(camera_device)
    }

    pub fn number_of_objects(&self) -> i32 {
        unsafe { wb_camera_recognition_get_number_of_objects(self.0) }
    }

    pub fn objects<'a>(&self) -> Result<Vec<RecognitionObject<'a>>, RecognitionError> {
        let number_of_objects = self.number_of_objects();
        let objects = unsafe {
            let objects = wb_camera_recognition_get_objects(self.0);
            if objects.is_null() {
                return Err(RecognitionError::ObjectsDataIsNull);
            }
            from_raw_parts(objects, number_of_objects as usize)
        };
        Ok(objects
            .iter()
            .map(|object| RecognitionObject {
                id: object.id,
                position: object.position,
                orientation: object.orientation,
                size: object.size,
                position_on_image: object.position_on_image,
                size_on_image: object.size_on_image,
                number_of_colors: object.number_of_colors,
                colors: unsafe { from_raw_parts(object.colors, object.number_of_colors as usize) },
                model: unsafe { CStr::from_ptr(object.model) }
                    .to_str()
                    .expect("CStr::to_str"),
            })
            .collect())
    }

    pub fn has_segmentation(&self) -> bool {
        unsafe { wb_camera_recognition_has_segmentation(self.0) != 0 }
    }

    pub fn enable_segmentation(&self) {
        unsafe { wb_camera_recognition_enable_segmentation(self.0) }
    }

    pub fn disable_segmentation(&self) {
        unsafe { wb_camera_recognition_disable_segmentation(self.0) }
    }

    pub fn is_segmentation_enabled(&self) -> bool {
        unsafe { wb_camera_recognition_is_segmentation_enabled(self.0) != 0 }
    }

    pub fn segmentation_image(&self) -> &[u8] {
        let camera = Camera::new(self.0);
        let width = camera.width();
        let height = camera.height();
        unsafe {
            let image = wb_camera_recognition_get_segmentation_image(self.0);
            from_raw_parts(image, (width * height * 4) as usize)
        }
    }

    pub fn save_segmentation_image(&self, filename: &str, quality: i32) -> i32 {
        let filename = CString::new(filename).expect("CString::new failed");
        unsafe { wb_camera_recognition_save_segmentation_image(self.0, filename.as_ptr(), quality) }
    }
}

impl Device for Recognition {
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

impl Sensor for Recognition {
    fn enable(&self, sampling_period: i32) {
        unsafe { wb_camera_recognition_enable(self.0, sampling_period) }
    }

    fn disable(&self) {
        unsafe { wb_camera_recognition_disable(self.0) }
    }

    fn sampling_period(&self) -> i32 {
        unsafe { wb_camera_recognition_get_sampling_period(self.0) }
    }

    fn set_sampling_period(&self, _sampling_period: i32) {
        unimplemented!()
    }
}
