/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

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
    pub fn _sceNpDrmGetRifName(
        rif_name: *mut crate::ctypes::c_char,
        aid: u64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceNpDrmGetFixedRifName(
        rif_name: *mut crate::ctypes::c_char,
        aid: u64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceNpDrmGetRifNameForInstall(
        rif_name: *mut crate::ctypes::c_char,
        rif_data: *const crate::ctypes::c_void,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
