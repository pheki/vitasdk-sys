/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

pub mod SceDisplayErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_DISPLAY_ERROR_OK: Type = 0;
    pub const SCE_DISPLAY_ERROR_INVALID_HEAD: Type = 2150170624;
    pub const SCE_DISPLAY_ERROR_INVALID_VALUE: Type = 2150170625;
    pub const SCE_DISPLAY_ERROR_INVALID_ADDR: Type = 2150170626;
    pub const SCE_DISPLAY_ERROR_INVALID_PIXELFORMAT: Type = 2150170627;
    pub const SCE_DISPLAY_ERROR_INVALID_PITCH: Type = 2150170628;
    pub const SCE_DISPLAY_ERROR_INVALID_RESOLUTION: Type = 2150170629;
    pub const SCE_DISPLAY_ERROR_INVALID_UPDATETIMING: Type = 2150170630;
    pub const SCE_DISPLAY_ERROR_NO_FRAME_BUFFER: Type = 2150170631;
    pub const SCE_DISPLAY_ERROR_NO_PIXEL_DATA: Type = 2150170632;
}
pub mod SceDisplayPixelFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_DISPLAY_PIXELFORMAT_A8B8G8R8: Type = 0;
    pub const SCE_DISPLAY_PIXELFORMAT_A2B10G10R10: Type = 1619001344;
}
pub mod SceDisplaySetBufSync {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_DISPLAY_SETBUF_IMMEDIATE: Type = 0;
    pub const SCE_DISPLAY_SETBUF_NEXTFRAME: Type = 1;
}
#[repr(C)]
pub struct SceDisplayFrameBuf {
    pub size: SceSize,
    pub base: *mut crate::ctypes::c_void,
    pub pitch: crate::ctypes::c_uint,
    pub pixelformat: crate::ctypes::c_uint,
    pub width: crate::ctypes::c_uint,
    pub height: crate::ctypes::c_uint,
}
#[repr(C)]
pub struct SceDisplayFrameBufInfo {
    pub size: SceSize,
    pub pid: SceUID,
    pub vblankcount: crate::ctypes::c_uint,
    pub paddr: usize,
    pub framebuf: SceDisplayFrameBuf,
    pub resolution: crate::ctypes::c_uint,
}
extern "C" {
    pub fn ksceDisplaySetFrameBuf(
        pParam: *const SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplaySetFrameBufInternal(
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        pParam: *const SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetFrameBuf(
        pParam: *mut SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetProcFrameBufInternal(
        pid: SceUID,
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        info: *mut SceDisplayFrameBufInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetMaximumFrameBufResolution(
        width: *mut crate::ctypes::c_int,
        height: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetPrimaryHead() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetVcountInternal(display: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStart() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartInternal(
        display: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartCB() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartCBInternal(
        display: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMultiInternal(
        display: crate::ctypes::c_int,
        vcount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMultiCB(vcount: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMultiCBInternal(
        display: crate::ctypes::c_int,
        vcount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBuf() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBufCB() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBufMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBufMultiCB(vcount: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterVblankStartCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayUnregisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayUnregisterVblankStartCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterFrameBufCallback(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterFrameBufCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplaySetInvertColors(
        display: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplaySetOwner(
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        pid: SceUID,
    ) -> crate::ctypes::c_int;
}
