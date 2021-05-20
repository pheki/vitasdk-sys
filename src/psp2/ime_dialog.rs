/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::common_dialog::*;
#[allow(unused_imports)]
use crate::psp2::types::*;

pub mod SceImeLanguage {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_LANGUAGE_DANISH: Type = 1;
    pub const SCE_IME_LANGUAGE_GERMAN: Type = 2;
    pub const SCE_IME_LANGUAGE_ENGLISH: Type = 4;
    pub const SCE_IME_LANGUAGE_SPANISH: Type = 8;
    pub const SCE_IME_LANGUAGE_FRENCH: Type = 16;
    pub const SCE_IME_LANGUAGE_ITALIAN: Type = 32;
    pub const SCE_IME_LANGUAGE_DUTCH: Type = 64;
    pub const SCE_IME_LANGUAGE_NORWEGIAN: Type = 128;
    pub const SCE_IME_LANGUAGE_POLISH: Type = 256;
    pub const SCE_IME_LANGUAGE_PORTUGUESE: Type = 512;
    pub const SCE_IME_LANGUAGE_RUSSIAN: Type = 1024;
    pub const SCE_IME_LANGUAGE_FINNISH: Type = 2048;
    pub const SCE_IME_LANGUAGE_SWEDISH: Type = 4096;
    pub const SCE_IME_LANGUAGE_JAPANESE: Type = 8192;
    pub const SCE_IME_LANGUAGE_KOREAN: Type = 16384;
    pub const SCE_IME_LANGUAGE_SIMPLIFIED_CHINESE: Type = 32768;
    pub const SCE_IME_LANGUAGE_TRADITIONAL_CHINESE: Type = 65536;
    pub const SCE_IME_LANGUAGE_PORTUGUESE_BR: Type = 131072;
    pub const SCE_IME_LANGUAGE_ENGLISH_GB: Type = 262144;
    pub const SCE_IME_LANGUAGE_TURKISH: Type = 524288;
}
pub mod SceImeType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_TYPE_DEFAULT: Type = 0;
    pub const SCE_IME_TYPE_BASIC_LATIN: Type = 1;
    pub const SCE_IME_TYPE_NUMBER: Type = 2;
    pub const SCE_IME_TYPE_EXTENDED_NUMBER: Type = 3;
}
pub mod SceImeEnterLabel {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_ENTER_LABEL_DEFAULT: Type = 0;
    pub const SCE_IME_ENTER_LABEL_SEND: Type = 1;
    pub const SCE_IME_ENTER_LABEL_SEARCH: Type = 2;
    pub const SCE_IME_ENTER_LABEL_GO: Type = 3;
}
pub mod SceImeOption {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_OPTION_MULTILINE: Type = 1;
    pub const SCE_IME_OPTION_NO_AUTO_CAPITALIZATION: Type = 2;
    pub const SCE_IME_OPTION_NO_ASSISTANCE: Type = 4;
}
pub mod SceImeDialogDialogMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_DIALOG_DIALOG_MODE_DEFAULT: Type = 0;
    pub const SCE_IME_DIALOG_DIALOG_MODE_WITH_CANCEL: Type = 1;
}
pub mod SceImeDialogTextboxMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_DIALOG_TEXTBOX_MODE_DEFAULT: Type = 0;
    pub const SCE_IME_DIALOG_TEXTBOX_MODE_PASSWORD: Type = 1;
    pub const SCE_IME_DIALOG_TEXTBOX_MODE_WITH_CLEAR: Type = 2;
}
pub mod SceImeDialogButton {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_DIALOG_BUTTON_NONE: Type = 0;
    pub const SCE_IME_DIALOG_BUTTON_CLOSE: Type = 1;
    pub const SCE_IME_DIALOG_BUTTON_ENTER: Type = 2;
}
pub type SceImeCharFilter =
    ::core::option::Option<unsafe extern "C" fn(ch: SceWChar16) -> SceInt32>;
#[repr(C)]
pub struct SceImeDialogParam {
    pub sdkVersion: SceUInt32,
    pub inputMethod: SceUInt32,
    pub supportedLanguages: SceUInt64,
    pub languagesForced: SceBool,
    pub type_: SceUInt32,
    pub option: SceUInt32,
    pub filter: SceImeCharFilter,
    pub dialogMode: SceUInt32,
    pub textBoxMode: SceUInt32,
    pub title: *const SceWChar16,
    pub maxTextLength: SceUInt32,
    pub initialText: *mut SceWChar16,
    pub inputTextBuffer: *mut SceWChar16,
    pub commonParam: SceCommonDialogParam,
    pub enterLabel: SceUChar8,
    pub reserved: [SceChar8; 35usize],
}
#[repr(C)]
pub struct SceImeDialogResult {
    pub result: SceInt32,
    pub button: SceInt32,
    pub reserved: [SceChar8; 28usize],
}
extern "C" {
    pub fn sceImeDialogInit(param: *const SceImeDialogParam) -> SceInt32;
}
extern "C" {
    pub fn sceImeDialogGetStatus() -> SceCommonDialogStatus::Type;
}
extern "C" {
    pub fn sceImeDialogAbort() -> SceInt32;
}
extern "C" {
    pub fn sceImeDialogGetResult(result: *mut SceImeDialogResult) -> SceInt32;
}
extern "C" {
    pub fn sceImeDialogTerm() -> SceInt32;
}