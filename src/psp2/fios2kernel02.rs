/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::fios2::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn sceFiosKernelOverlayAddForProcess02(
        pid: SceUID,
        overlay: *mut SceFiosOverlay,
        outID: *mut SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
}
