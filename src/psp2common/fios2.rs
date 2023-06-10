/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;

pub const SCE_FIOS2_OVERLAY_PATH_SIZE: u32 = 292;
pub const SCE_FIOS2_OVERLAY_PATH_MAX_LENGTH: u32 = 291;
pub type SceFiosOverlayID = i32;
pub mod SceFiosOverlayType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FIOS_OVERLAY_TYPE_OPAQUE: Type = 0;
    pub const SCE_FIOS_OVERLAY_TYPE_TRANSLUCENT: Type = 1;
    pub const SCE_FIOS_OVERLAY_TYPE_NEWER: Type = 2;
    pub const SCE_FIOS_OVERLAY_TYPE_WRITABLE: Type = 3;
}
#[repr(C)]
pub struct SceFiosOverlay {
    pub type_: u8,
    pub order: u8,
    pub dst_len: u16,
    pub src_len: u16,
    pub unk2: u16,
    pub pid: SceUID,
    pub id: SceFiosOverlayID,
    pub dst: [crate::ctypes::c_char; 292usize],
    pub src: [crate::ctypes::c_char; 292usize],
}
