/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::net::net::*;
#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSyscallParameter {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sceNetSyscallAccept(
        s: crate::ctypes::c_int,
        addr: *mut crate::ctypes::c_void,
        addrlen: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallBind(
        s: crate::ctypes::c_int,
        addr: *const crate::ctypes::c_void,
        addrlen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallClose(s: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallConnect(
        s: crate::ctypes::c_int,
        name: *const crate::ctypes::c_void,
        namelen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallControl(
        if_index: crate::ctypes::c_int,
        code: crate::ctypes::c_int,
        ptr: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDescriptorClose(id: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDescriptorCreate(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDescriptorCtl(
        id: crate::ctypes::c_int,
        op: crate::ctypes::c_int,
        s: crate::ctypes::c_int,
        info: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDumpAbort(
        id: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDumpClose(id: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDumpCreate(
        name: *const crate::ctypes::c_char,
        len: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallDumpRead(
        id: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_int,
        pflags: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallEpollAbort(
        eid: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallEpollClose(eid: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallEpollCreate(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallEpollCtl(
        eid: crate::ctypes::c_int,
        op: crate::ctypes::c_int,
        id: crate::ctypes::c_int,
        event: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallEpollWait(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallGetIfList(
        list: *mut crate::ctypes::c_void,
        n: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallGetSockinfo(
        s: crate::ctypes::c_int,
        ptr: *mut crate::ctypes::c_void,
        n: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallGetpeername(
        s: crate::ctypes::c_int,
        name: *mut crate::ctypes::c_void,
        namelen: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallGetsockname(
        s: crate::ctypes::c_int,
        name: *mut crate::ctypes::c_void,
        namelen: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallGetsockopt(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallIcmConnect(
        s: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallIoctl(
        s: crate::ctypes::c_int,
        com: crate::ctypes::c_uint,
        data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallListen(
        s: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallRecvfrom(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallRecvmsg(
        s: crate::ctypes::c_int,
        msg: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallSendmsg(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallSendto(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallSetsockopt(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallShutdown(
        s: crate::ctypes::c_int,
        how: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallSocket(
        name: *const crate::ctypes::c_char,
        domain: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        protocol: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallSocketAbort(
        s: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetSyscallSysctl(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
