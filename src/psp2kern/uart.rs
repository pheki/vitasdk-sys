/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn ksceUartInit(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUartReadAvailable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUartRead(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUartWrite(
        port: crate::ctypes::c_int,
        data: crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}