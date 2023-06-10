/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;

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
#[repr(C)]
pub struct ScePsmDrmLicense {
    pub magic: [crate::ctypes::c_char; 8usize],
    pub unk1: SceUInt32,
    pub unk2: SceUInt32,
    pub account_id: SceUInt64,
    pub unk3: SceUInt32,
    pub unk4: SceUInt32,
    pub start_time: SceUInt64,
    pub expiration_time: SceUInt64,
    pub activation_checksum: [SceUInt8; 32usize],
    pub content_id: [crate::ctypes::c_char; 48usize],
    pub unk5: [SceUInt8; 128usize],
    pub unk6: [SceUInt8; 32usize],
    pub key: [SceUInt8; 16usize],
    pub signature: [SceUInt8; 464usize],
    pub rsa_signature: [SceUInt8; 256usize],
}
