use std::ffi::{c_char, c_void};

use webots_bindings::{
    wb_device_get_model, wb_device_get_name, wb_device_get_node_type, wb_display_attach_camera,
    wb_display_detach_camera, wb_display_draw_line, wb_display_draw_oval, wb_display_draw_pixel,
    wb_display_draw_polygon, wb_display_draw_rectangle, wb_display_draw_text, wb_display_fill_oval,
    wb_display_fill_polygon, wb_display_fill_rectangle, wb_display_get_height,
    wb_display_get_width, wb_display_image_copy, wb_display_image_delete, wb_display_image_load,
    wb_display_image_new, wb_display_image_paste, wb_display_image_save, wb_display_set_alpha,
    wb_display_set_color, wb_display_set_font, wb_display_set_opacity, WbDeviceTag, WbImageRef,
    WbNodeType_WB_NODE_DISPLAY,
};

use crate::{Camera, Device};

pub struct ImageRef(WbImageRef);

pub struct Display(WbDeviceTag);

impl Display {
    pub fn new(tag: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_DISPLAY, unsafe {
            wb_device_get_node_type(tag)
        });
        Self(tag)
    }
    pub fn attach_camera(&self, camera: &Camera) {
        unsafe {
            wb_display_attach_camera(self.0, camera.tag());
        }
    }

    pub fn detach_camera(&self) {
        unsafe {
            wb_display_detach_camera(self.0);
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            wb_display_draw_line(self.0, x1, y1, x2, y2);
        }
    }

    pub fn draw_oval(&self, cx: i32, cy: i32, a: i32, b: i32) {
        unsafe {
            wb_display_draw_oval(self.0, cx, cy, a, b);
        }
    }

    pub fn draw_pixel(&self, x: i32, y: i32) {
        unsafe {
            wb_display_draw_pixel(self.0, x, y);
        }
    }

    pub fn draw_polygon(&self, x: &[i32], y: &[i32]) {
        unsafe {
            wb_display_draw_polygon(self.0, x.as_ptr(), y.as_ptr(), x.len().min(y.len()) as i32)
        }
    }

    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { wb_display_draw_rectangle(self.0, x, y, width, height) }
    }

    pub fn draw_text(&self, text: &str, x: i32, y: i32) {
        unsafe { wb_display_draw_text(self.0, text.as_ptr() as *const c_char, x, y) }
    }

    pub fn fill_oval(&self, cx: i32, cy: i32, a: i32, b: i32) {
        unsafe { wb_display_fill_oval(self.0, cx, cy, a, b) }
    }

    pub fn fill_polygon(&self, x: &[i32], y: &[i32]) {
        unsafe {
            wb_display_fill_polygon(self.0, x.as_ptr(), y.as_ptr(), x.len().min(y.len()) as i32)
        }
    }

    pub fn fill_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { wb_display_fill_rectangle(self.0, x, y, width, height) }
    }

    pub fn height(&self) -> i32 {
        unsafe { wb_display_get_height(self.0) }
    }

    pub fn width(&self) -> i32 {
        unsafe { wb_display_get_width(self.0) }
    }

    pub fn image_copy(&self, x: i32, y: i32, width: i32, height: i32) -> WbImageRef {
        unsafe { wb_display_image_copy(self.0, x, y, width, height) }
    }

    pub fn image_delete(&self, image: ImageRef) {
        unsafe { wb_display_image_delete(self.0, image.0) }
    }

    pub fn image_new(&self, data: &[u8], format: i32, width: i32, height: i32) -> WbImageRef {
        unsafe {
            wb_display_image_new(
                self.0,
                width,
                height,
                data.as_ptr() as *const c_void,
                format,
            )
        }
    }

    pub fn image_load(&self, filename: &str) -> ImageRef {
        ImageRef(unsafe { wb_display_image_load(self.0, filename.as_ptr() as *const c_char) })
    }

    pub fn image_paste(&self, image: ImageRef, x: i32, y: i32, blend: bool) {
        unsafe { wb_display_image_paste(self.0, image.0, x, y, blend as c_char) }
    }

    pub fn image_save(&self, image: ImageRef, filename: &str) {
        unsafe { wb_display_image_save(self.0, image.0, filename.as_ptr() as *const c_char) }
    }

    pub fn set_alpha(&self, alpha: f64) {
        unsafe { wb_display_set_alpha(self.0, alpha) }
    }

    pub fn set_color(&self, color: i32) {
        unsafe { wb_display_set_color(self.0, color) }
    }

    pub fn set_font(&self, font: &str, size: i32, anti_aliasing: bool) {
        unsafe {
            wb_display_set_font(
                self.0,
                font.as_ptr() as *const c_char,
                size,
                anti_aliasing as c_char,
            )
        }
    }

    pub fn set_opacity(&self, opacity: f64) {
        unsafe { wb_display_set_opacity(self.0, opacity) }
    }
}

impl Device for Display {
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
