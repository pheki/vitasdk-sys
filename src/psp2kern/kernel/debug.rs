/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const __GNUC_VA_LIST: u32 = 1;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
pub struct SceKernelDebugMessageContext {
    pub hex_value0_hi: SceUInt32,
    pub hex_value0_lo: SceUInt32,
    pub hex_value1: SceUInt32,
    pub func: *const crate::ctypes::c_char,
    pub line: SceSize,
    pub file: *const crate::ctypes::c_char,
}
pub mod SceKernelDebugPrintFlags {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_DBG_PRINT_FLAG_NONE: Type = 0;
    pub const SCE_DBG_PRINT_FLAG_CORE: Type = 1;
    pub const SCE_DBG_PRINT_FLAG_FUNC: Type = 2;
    pub const SCE_DBG_PRINT_FLAG_LINE: Type = 4;
    pub const SCE_DBG_PRINT_FLAG_FILE: Type = 8;
}
extern "C" {
    pub fn ksceDebugPutchar(character: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugPrintf(fmt: *const crate::ctypes::c_char, ...) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugPrintf2(
        flags: crate::ctypes::c_int,
        ctx: *const SceKernelDebugMessageContext,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugPrintKernelPanic(
        ctx: *const SceKernelDebugMessageContext,
        lr: *const crate::ctypes::c_void,
    );
}
extern "C" {
    pub fn ksceDebugPrintfKernelPanic(
        ctx: *const SceKernelDebugMessageContext,
        lr: *const crate::ctypes::c_void,
        fmt: *const crate::ctypes::c_char,
        ...
    );
}
extern "C" {
    pub fn ksceDebugPrintKernelAssertion(
        condition: crate::ctypes::c_int,
        ctx: *const SceKernelDebugMessageContext,
        lr: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugPrintfKernelAssertion(
        level: crate::ctypes::c_int,
        condition: crate::ctypes::c_int,
        ctx: *const SceKernelDebugMessageContext,
        lr: *const crate::ctypes::c_void,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugSetHandlers(
        func: ::core::option::Option<
            unsafe extern "C" fn(
                unk: crate::ctypes::c_int,
                format: *const crate::ctypes::c_char,
                args: va_list,
            ) -> crate::ctypes::c_int,
        >,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugRegisterPutcharHandler(
        func: ::core::option::Option<
            unsafe extern "C" fn(
                args: *mut crate::ctypes::c_void,
                c: crate::ctypes::c_char,
            ) -> crate::ctypes::c_int,
        >,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDebugGetPutcharHandler() -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn ksceDebugDisableInfoDump(flag: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[repr(C, packed)]
pub struct SceKernelDebugEventLog1 {
    pub data_0x40: crate::ctypes::c_int,
    pub pid: SceUID,
    pub budget_type: crate::ctypes::c_int,
    pub data_0x4C: crate::ctypes::c_int,
    pub titleid: [crate::ctypes::c_char; 12usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugEventLog2 {
    pub data_0x40: crate::ctypes::c_int,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugEventLog3 {
    pub data_0x40: crate::ctypes::c_int,
    pub ip1: [crate::ctypes::c_char; 16usize],
    pub ip2: [crate::ctypes::c_char; 16usize],
    pub ip3: [crate::ctypes::c_char; 16usize],
    pub ip4: [crate::ctypes::c_char; 16usize],
    pub ip5: [crate::ctypes::c_char; 16usize],
}
#[repr(C, packed)]
pub struct SceKernelDebugEventLog {
    pub size: SceSize,
    pub data_0x04: crate::ctypes::c_int,
    pub titleid: [crate::ctypes::c_char; 12usize],
    pub flags: crate::ctypes::c_int,
    pub ppid: SceUID,
    pub data_0x1C: SceUID,
    pub rsvd: [crate::ctypes::c_int; 4usize],
    pub time: SceUInt64,
    pub data_0x38: crate::ctypes::c_int,
    pub item_size: SceSize,
    pub __bindgen_anon_1: SceKernelDebugEventLog__bindgen_ty_1,
}
#[repr(C)]
pub struct SceKernelDebugEventLog__bindgen_ty_1 {
    pub type1: __BindgenUnionField<SceKernelDebugEventLog1>,
    pub type2: __BindgenUnionField<SceKernelDebugEventLog2>,
    pub type3: __BindgenUnionField<SceKernelDebugEventLog3>,
    pub bindgen_union_field: [u8; 84usize],
}
pub type __builtin_va_list = __va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list {
    pub __ap: *mut crate::ctypes::c_void,
}
