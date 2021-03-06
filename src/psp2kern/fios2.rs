/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

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
extern "C" {
    pub fn ksceFiosKernelOverlayAdd(
        overlay: *mut SceFiosOverlay,
        outID: *mut SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceFiosKernelOverlayAddForProcess(
        pid: SceUID,
        overlay: *mut SceFiosOverlay,
        outID: *mut SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceFiosKernelOverlayRemoveForProcess(
        pid: SceUID,
        id: SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceFiosKernelOverlayResolveSync(
        pid: SceUID,
        resolveFlag: crate::ctypes::c_int,
        inPath: *const crate::ctypes::c_char,
        outPath: *mut crate::ctypes::c_char,
        maxPath: SceSize,
    ) -> crate::ctypes::c_int;
}
