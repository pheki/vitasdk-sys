/* automatically generated by rust-bindgen 0.65.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelOpenPsId {
    pub id: [crate::ctypes::c_char; 16usize],
}
extern "C" {
    pub fn sceKernelGetOpenPsId(id: *mut SceKernelOpenPsId) -> crate::ctypes::c_int;
}
