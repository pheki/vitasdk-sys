/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::kernel::iofilemgr::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

extern "C" {
    pub fn sceIoOpen(
        file: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        mode: SceMode,
    ) -> SceUID;
}
extern "C" {
    pub fn sceIoClose(fd: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoRead(fd: SceUID, buf: *mut crate::ctypes::c_void, nbyte: SceSize) -> SceSSize;
}
extern "C" {
    pub fn sceIoPread(
        fd: SceUID,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoWrite(fd: SceUID, buf: *const crate::ctypes::c_void, nbyte: SceSize) -> SceSSize;
}
extern "C" {
    pub fn sceIoPwrite(
        fd: SceUID,
        data: *const crate::ctypes::c_void,
        size: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoLseek(fd: SceUID, offset: SceOff, whence: crate::ctypes::c_int) -> SceOff;
}
extern "C" {
    pub fn sceIoLseek32(
        fd: SceUID,
        offset: crate::ctypes::c_long,
        whence: crate::ctypes::c_int,
    ) -> crate::ctypes::c_long;
}
extern "C" {
    pub fn sceIoRemove(file: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoRename(
        oldname: *const crate::ctypes::c_char,
        newname: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoSync(
        device: *const crate::ctypes::c_char,
        unk: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoSyncByFd(fd: SceUID, flag: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoCancel(fd: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoGetPriority(fd: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoGetProcessDefaultPriority() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoGetThreadDefaultPriority() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoSetPriority(fd: SceUID, priority: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoSetProcessDefaultPriority(priority: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceIoSetThreadDefaultPriority(priority: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
