use crate::bindings::*;

pub mod containers;
pub mod custom;
pub mod image;
pub mod rectangle;
pub mod text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElementConfigType {
    Rectangle = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE,
    BorderContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER,
    FloatingContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER,
    ScrollContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER,
    Image = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE,
    Text = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT,
    Custom = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM,
    // Rust specific enum types (uses the same approach as Odin bindings)
    Id = 65,
    Layout = 66,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum PointerCaptureMode {
    Capture = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE,
    Passthrough = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Dimensions {
    width: f32,
    height: f32
}

impl From<Clay_Dimensions> for Dimensions {
    fn from(value: Clay_Dimensions) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl From<Dimensions> for Clay_Dimensions {
    fn from(value: Dimensions) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}