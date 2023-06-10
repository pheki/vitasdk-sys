/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

pub type va_list = u32;
extern "C" {
    pub fn sce_paf_malloc(size: SceSize) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_free(ptr: *mut crate::ctypes::c_void);
}
extern "C" {
    pub fn sce_paf_memalign(align: SceSize, length: SceSize) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_memchr(
        src: *const crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        length: SceSize,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_memcmp(
        s1: *const crate::ctypes::c_void,
        s2: *const crate::ctypes::c_void,
        n: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_memcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_memset(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_memmove(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_snprintf(
        dst: *mut crate::ctypes::c_char,
        max: crate::ctypes::c_uint,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_vsnprintf(
        dst: *mut crate::ctypes::c_char,
        max: crate::ctypes::c_uint,
        fmt: *const crate::ctypes::c_char,
        arg: va_list,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_bcmp(
        ptr1: *const crate::ctypes::c_void,
        ptr2: *const crate::ctypes::c_void,
        num: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_bcopy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        n: SceSize,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_bzero(dst: *mut crate::ctypes::c_void, n: SceSize)
        -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn sce_paf_strchr(
        s: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
}
extern "C" {
    pub fn sce_paf_strcmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_strlen(s: *const crate::ctypes::c_char) -> usize;
}
extern "C" {
    pub fn sce_paf_strcasecmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_strncasecmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_strncmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sce_paf_strncpy(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> *mut crate::ctypes::c_char;
}
extern "C" {
    pub fn sce_paf_strrchr(
        s: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
}
extern "C" {
    pub fn sce_paf_strtod(
        nptr: *const crate::ctypes::c_char,
        endptr: *mut *mut crate::ctypes::c_char,
    ) -> f64;
}
