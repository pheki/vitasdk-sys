/* automatically generated by rust-bindgen 0.65.1 */

pub mod SceBgAppUtilErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_BGAPP_UTIL_ERROR_INVALID_ARG: Type = 2148558082;
}
extern "C" {
    pub fn sceBgAppUtilStartBgApp(mode: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
