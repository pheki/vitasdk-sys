/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

#[repr(C)]
pub struct SceNpDrmLicense {
    pub version: SceInt16,
    pub version_flags: SceInt16,
    pub license_type: SceInt16,
    pub license_flags: SceInt16,
    pub account_id: SceUInt64,
    pub content_id: [crate::ctypes::c_char; 48usize],
    pub key_table: [crate::ctypes::c_char; 16usize],
    pub key1: [crate::ctypes::c_char; 16usize],
    pub start_time: SceInt64,
    pub expiration_time: SceInt64,
    pub ecdsa_signature: [crate::ctypes::c_char; 40usize],
    pub flags: SceInt64,
    pub key2: [crate::ctypes::c_char; 16usize],
    pub unk_0xB0: [crate::ctypes::c_char; 16usize],
    pub open_psid: [crate::ctypes::c_char; 16usize],
    pub unk_0xD0: [crate::ctypes::c_char; 16usize],
    pub cmd56_handshake_part: [crate::ctypes::c_char; 20usize],
    pub debug_upgradable: crate::ctypes::c_int,
    pub unk_0xF8: crate::ctypes::c_int,
    pub sku_flag: crate::ctypes::c_int,
    pub rsa_signature: [crate::ctypes::c_char; 256usize],
}
extern "C" {
    pub fn ksceNpDrmGetRifName(
        name: *mut crate::ctypes::c_char,
        aid: SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetFixedRifName(
        name: *mut crate::ctypes::c_char,
        aid: SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetRifVitaKey(
        license: *const crate::ctypes::c_void,
        klicense: *mut crate::ctypes::c_void,
        flags: *mut crate::ctypes::c_int,
        sku_flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceUInt64,
        lic_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetRifInfo(
        license: *const crate::ctypes::c_void,
        license_size: SceSize,
        check_sign: crate::ctypes::c_int,
        content_id: *mut crate::ctypes::c_char,
        account_id: *mut SceUInt64,
        license_version: *mut crate::ctypes::c_int,
        license_flags: *mut crate::ctypes::c_int,
        flags: *mut crate::ctypes::c_int,
        sku_flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceInt64,
        lic_exp_time: *mut SceInt64,
        flags2: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
}