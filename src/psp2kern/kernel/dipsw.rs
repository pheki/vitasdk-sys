/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

#[repr(C)]
pub struct SceDipsw {
    pub cp_timestamp_1: u32,
    pub cp_version: u16,
    pub cp_build_id: u16,
    pub cp_timestamp_2: u32,
    pub aslr_seed: u32,
    pub sce_sdk_flags: u32,
    pub shell_flags: u32,
    pub debug_control_flags: u32,
    pub system_control_flags: u32,
}
