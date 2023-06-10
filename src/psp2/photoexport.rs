/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhotoExportParam {
    pub version: crate::ctypes::c_int,
    pub photoTitle: *const SceWChar32,
    pub gameTitle: *const SceWChar32,
    pub gameComment: *const SceWChar32,
    pub reserved: [crate::ctypes::c_int; 8usize],
}
extern "C" {
    pub fn scePhotoExportFromData(
        data: *const crate::ctypes::c_void,
        size: SceSize,
        param: *const PhotoExportParam,
        workingMemory: *mut crate::ctypes::c_void,
        cancelCb: *mut crate::ctypes::c_void,
        user: *mut crate::ctypes::c_void,
        outPath: *mut crate::ctypes::c_char,
        outPathSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePhotoExportFromFile(
        path: *const crate::ctypes::c_char,
        param: *const PhotoExportParam,
        workingMemory: *mut crate::ctypes::c_void,
        cancelCb: *mut crate::ctypes::c_void,
        user: *mut crate::ctypes::c_void,
        outPath: *mut crate::ctypes::c_char,
        outPathSize: SceSize,
    ) -> crate::ctypes::c_int;
}
