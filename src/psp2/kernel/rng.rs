/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

extern "C" {
    pub fn sceKernelGetRandomNumber(
        output: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
