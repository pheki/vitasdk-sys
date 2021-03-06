/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub const SCE_KERNEL_PROCESS_ID_SELF: u32 = 0;
pub mod SceKernelMemBlockType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE: Type = 203456608;
    pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RX: Type = 203477072;
    pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RW: Type = 203477088;
    pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_RW: Type = 209768544;
    pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_RW: Type = 226525280;
    pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW: Type = 155222112;
}
pub mod SceKernelAllocMemBlockAttr {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_ALIGNMENT: Type = 4;
}
#[repr(C)]
pub struct SceKernelAllocMemBlockOpt {
    pub size: SceSize,
    pub attr: SceUInt32,
    pub alignment: SceSize,
    pub uidBaseBlock: SceUInt32,
    pub strBaseBlockName: *const crate::ctypes::c_char,
    pub flags: crate::ctypes::c_int,
    pub reserved: [crate::ctypes::c_int; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelFreeMemorySizeInfo {
    pub size: crate::ctypes::c_int,
    pub size_user: crate::ctypes::c_int,
    pub size_cdram: crate::ctypes::c_int,
    pub size_phycont: crate::ctypes::c_int,
}
pub mod SceKernelModel {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_MODEL_VITA: Type = 65536;
    pub const SCE_KERNEL_MODEL_VITATV: Type = 131072;
}
#[repr(C)]
pub struct SceKernelMemBlockInfo {
    pub size: SceSize,
    pub mappedBase: *mut crate::ctypes::c_void,
    pub mappedSize: SceSize,
    pub memoryType: crate::ctypes::c_int,
    pub access: SceUInt32,
    pub type_: SceKernelMemBlockType::Type,
}
pub mod SceKernelMemoryAccessType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_MEMORY_ACCESS_X: Type = 1;
    pub const SCE_KERNEL_MEMORY_ACCESS_W: Type = 2;
    pub const SCE_KERNEL_MEMORY_ACCESS_R: Type = 4;
}
pub mod SceKernelMemoryType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_MEMORY_TYPE_NORMAL_NC: Type = 128;
    pub const SCE_KERNEL_MEMORY_TYPE_NORMAL: Type = 208;
}
extern "C" {
    pub fn sceKernelAllocMemBlock(
        name: *const crate::ctypes::c_char,
        type_: SceKernelMemBlockType::Type,
        size: SceSize,
        opt: *mut SceKernelAllocMemBlockOpt,
    ) -> SceUID;
}
extern "C" {
    pub fn sceKernelFreeMemBlock(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetMemBlockBase(
        uid: SceUID,
        base: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelFindMemBlockByAddr(addr: *const crate::ctypes::c_void, size: SceSize)
        -> SceUID;
}
extern "C" {
    pub fn sceKernelGetMemBlockInfoByAddr(
        base: *mut crate::ctypes::c_void,
        info: *mut SceKernelMemBlockInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetMemBlockInfoByRange(
        base: *mut crate::ctypes::c_void,
        size: SceSize,
        info: *mut SceKernelMemBlockInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelAllocMemBlockForVM(name: *const crate::ctypes::c_char, size: SceSize)
        -> SceUID;
}
extern "C" {
    pub fn sceKernelSyncVMDomain(
        uid: SceUID,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelOpenVMDomain() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelCloseVMDomain() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelOpenMemBlock(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelCloseMemBlock(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetModelForCDialog() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetModel() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetFreeMemorySize(
        info: *mut SceKernelFreeMemorySizeInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelIsPSVitaTV() -> crate::ctypes::c_int;
}
