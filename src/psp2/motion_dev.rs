/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevAccCalibData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevCalibrationData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevCalibrationHeader {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevMagnCalibData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevDeviceInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevDeviceLocation {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevGyroBiasData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevGyroCalibData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevModeInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevData {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sceMotionDevGetAccCalibData(data: *mut SceMotionDevAccCalibData)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetAccCalibData2(
        port: crate::ctypes::c_int,
        data: *mut SceMotionDevAccCalibData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetCalibrationData(
        block_id: SceUInt32,
        data: *mut SceMotionDevCalibrationData,
        data_num: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetCalibrationHeader(
        block_id: SceUInt32,
        calib_header: *mut SceMotionDevCalibrationHeader,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetCurrentMagnCalibData(
        data: *mut SceMotionDevMagnCalibData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetCurrentMagnStabilityLevel(level: *mut SceUInt32) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetDeviceInfo(
        device_info: *mut SceMotionDevDeviceInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetDeviceLocation(
        location: *mut SceMotionDevDeviceLocation,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetFactoryMagnCalibData(
        data: *mut SceMotionDevMagnCalibData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetGyroBias(bias: *mut SceMotionDevGyroBiasData) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetGyroBias2(
        port: crate::ctypes::c_int,
        bias: *mut SceMotionDevGyroBiasData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetGyroCalibData(
        data: *mut SceMotionDevGyroCalibData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetGyroCalibData2(
        port: crate::ctypes::c_int,
        data: *mut SceMotionDevGyroCalibData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevGetMeasMode(mode_info: *mut SceMotionDevModeInfo) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevIsReady() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevMagnSamplingStart() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevMagnSamplingStop() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevRead(
        data: *mut SceMotionDevData,
        data_num: crate::ctypes::c_int,
        info: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevRead2(
        port: crate::ctypes::c_int,
        data: *mut SceMotionDevData,
        data_num: crate::ctypes::c_int,
        info: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevReadForMagnCalib(
        data: *mut SceMotionDevData,
        data_num: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevSamplingStart() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevSamplingStart2(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevSamplingStop() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevSamplingStop2(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevSetSamplingMode(mode: SceUInt32) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevUpdateMagnCalibData(
        data: *const SceMotionDevMagnCalibData,
        tag: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceMotionDevUpdateMagnStabilityLevel(level: SceUInt32) -> crate::ctypes::c_int;
}
