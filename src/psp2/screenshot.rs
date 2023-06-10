/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

pub const SCE_SCREENSHOT_MAX_FS_PATH: u32 = 1024;
pub const SCE_SCREENSHOT_MAX_PHOTO_TITLE_LEN: u32 = 64;
pub const SCE_SCREENSHOT_MAX_PHOTO_TITLE_SIZE: u32 = 256;
pub const SCE_SCREENSHOT_MAX_GAME_TITLE_LEN: u32 = 64;
pub const SCE_SCREENSHOT_MAX_GAME_TITLE_SIZE: u32 = 256;
pub const SCE_SCREENSHOT_MAX_GAME_COMMENT_LEN: u32 = 128;
pub const SCE_SCREENSHOT_MAX_GAME_COMMENT_SIZE: u32 = 512;
pub mod SceScreenshotErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SCREENSHOT_ERROR_INVALID_ARGUMENT: Type = 2148544257;
    pub const SCE_SCREENSHOT_ERROR_NO_MEMORY: Type = 2148544258;
    pub const SCE_SCREENSHOT_ERROR_FILE_NOT_FOUND: Type = 2148544259;
    pub const SCE_SCREENSHOT_ERROR_NOT_SUPPORTED_FORMAT: Type = 2148544260;
    pub const SCE_SCREENSHOT_ERROR_MEDIA_FULL: Type = 2148544261;
    pub const SCE_SCREENSHOT_ERROR_INTERNAL: Type = 2148544262;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceScreenShotParam {
    pub photoTitle: *const SceWChar32,
    pub gameTitle: *const SceWChar32,
    pub gameComment: *const SceWChar32,
    pub reserved: *mut crate::ctypes::c_void,
}
extern "C" {
    pub fn sceScreenShotSetParam(param: *const SceScreenShotParam) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceScreenShotSetOverlayImage(
        filepath: *const crate::ctypes::c_char,
        offsetX: crate::ctypes::c_int,
        offsetY: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceScreenShotDisable() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceScreenShotEnable() -> crate::ctypes::c_int;
}
