/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

pub mod SceI2cErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_I2C_ERROR_INVALID_BUS: Type = 2151613184;
    pub const SCE_I2C_ERROR_INVALID_SIZE: Type = 2151613186;
    pub const SCE_I2C_ERROR_INVALID_ADDR: Type = 2151613187;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceI2cDebugHandlers {
    pub size: crate::ctypes::c_uint,
    pub write_start: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            addr: crate::ctypes::c_int,
            buffer: *mut crate::ctypes::c_uchar,
            size: crate::ctypes::c_int,
        ),
    >,
    pub write_end: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            error: crate::ctypes::c_int,
            result: crate::ctypes::c_int,
        ),
    >,
    pub read_start: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            addr: crate::ctypes::c_int,
            buffer: *mut crate::ctypes::c_uchar,
            size: crate::ctypes::c_int,
        ),
    >,
    pub read_end: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            error: crate::ctypes::c_int,
            result: crate::ctypes::c_int,
        ),
    >,
    pub write_read_start: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            write_addr: crate::ctypes::c_int,
            write_buffer: *mut crate::ctypes::c_uchar,
            write_size: crate::ctypes::c_int,
            read_addr: crate::ctypes::c_uint,
            read_buffer: *mut crate::ctypes::c_uchar,
            read_size: crate::ctypes::c_int,
        ),
    >,
    pub write_read_end: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            error: crate::ctypes::c_int,
            result: crate::ctypes::c_int,
        ),
    >,
}
extern "C" {
    pub fn ksceI2cInit(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceI2cReset(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceI2cTransferRead(
        bus: crate::ctypes::c_int,
        addr: crate::ctypes::c_uint,
        buffer: *mut crate::ctypes::c_uchar,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceI2cTransferWrite(
        bus: crate::ctypes::c_int,
        addr: crate::ctypes::c_uint,
        buffer: *const crate::ctypes::c_uchar,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceI2cTransferWriteRead(
        bus: crate::ctypes::c_int,
        write_addr: crate::ctypes::c_uint,
        write_buffer: *mut crate::ctypes::c_uchar,
        write_size: crate::ctypes::c_int,
        read_addr: crate::ctypes::c_uint,
        read_buffer: *mut crate::ctypes::c_uchar,
        read_size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceI2cSetDebugHandlers(
        bus: crate::ctypes::c_int,
        debug_handlers: *mut SceI2cDebugHandlers,
    ) -> crate::ctypes::c_int;
}