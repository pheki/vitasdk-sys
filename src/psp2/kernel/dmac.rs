/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

extern "C" {
    pub fn sceDmacMemcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceDmacMemset(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
