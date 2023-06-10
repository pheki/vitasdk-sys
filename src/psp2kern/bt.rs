/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

pub mod SceBtErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_BT_ERROR_REG_NOT_READY: Type = 2150564097;
    pub const SCE_BT_ERROR_REG_DELETE_NO_ENTRY: Type = 2150564098;
    pub const SCE_BT_ERROR_REG_UPDATE_CANNOT_SAVE: Type = 2150564099;
    pub const SCE_BT_ERROR_REG_SET_HID_DESC_BAD_ARG: Type = 2150564101;
    pub const SCE_BT_ERROR_REG_SET_HID_DESC_NO_REG: Type = 2150564102;
    pub const SCE_BT_ERROR_REG_GET_HID_DESC_NO_REG: Type = 2150564103;
    pub const SCE_BT_ERROR_REG_GET_HID_DESC_TOO_SHORT: Type = 2150564104;
    pub const SCE_BT_ERROR_REG_DELETE_CONNECTING: Type = 2150564105;
    pub const SCE_BT_ERROR_REG_CANNOT_LOAD: Type = 2150564106;
    pub const SCE_BT_ERROR_REG_CANNOT_OPEN: Type = 2150564107;
    pub const SCE_BT_ERROR_REG_CANNOT_READ: Type = 2150564108;
    pub const SCE_BT_ERROR_REG_CANNOT_WRITE: Type = 2150564109;
    pub const SCE_BT_ERROR_INQUIRY_START_BUSY: Type = 2150564353;
    pub const SCE_BT_ERROR_CONNECT_START_NO_REG: Type = 2150564354;
    pub const SCE_BT_ERROR_CONNECT_START_NOT_CONNECTABLE: Type = 2150564355;
    pub const SCE_BT_ERROR_CONNECT_START_BUSY: Type = 2150564356;
    pub const SCE_BT_ERROR_DISCONNECT_START_NOT_CONNECTED: Type = 2150564357;
    pub const SCE_BT_ERROR_PIN_INVALID_LENGTH: Type = 2150564358;
    pub const SCE_BT_ERROR_USER_CONFIRM_NOT_CONNECTED: Type = 2150564359;
    pub const SCE_BT_ERROR_SDP_OPEN_NO_L2C: Type = 2150564360;
    pub const SCE_BT_ERROR_CONNECT_START_REG_FULL: Type = 2150564361;
    pub const SCE_BT_ERROR_CONNECT_START_CONNECTED: Type = 2150564362;
    pub const SCE_BT_ERROR_CONNECT_START_TOO_MANY: Type = 2150564365;
    pub const SCE_BT_ERROR_CONNECT_START_DELETING: Type = 2150564366;
    pub const SCE_BT_ERROR_AVDTP_OPEN_NO_L2C: Type = 2150565121;
    pub const SCE_BT_ERROR_AVDTP_CLOSE_BAD_SERV: Type = 2150565122;
    pub const SCE_BT_ERROR_AVDTP_CLOSE_BAD_STATE: Type = 2150565123;
    pub const SCE_BT_ERROR_AVDTP_START_BAD_SERV: Type = 2150565124;
    pub const SCE_BT_ERROR_AVDTP_START_BAD_STATE: Type = 2150565125;
    pub const SCE_BT_ERROR_AVDTP_STOP_BAD_SERV: Type = 2150565126;
    pub const SCE_BT_ERROR_AVDTP_STOP_BAD_STATE: Type = 2150565127;
    pub const SCE_BT_ERROR_AVDTP_SEND_BAD_STATE: Type = 2150565128;
    pub const SCE_BT_ERROR_AVDTP_RECONF_BAD_SERV: Type = 2150565129;
    pub const SCE_BT_ERROR_AVDTP_RECONF_BAD_STATE: Type = 2150565130;
    pub const SCE_BT_ERROR_AVCTP_OPEN_NO_L2C: Type = 2150565377;
    pub const SCE_BT_ERROR_AVCTP_SEND_NO_L2C: Type = 2150565379;
    pub const SCE_BT_ERROR_AVCTP_NOT_CONNECTED: Type = 2150565380;
    pub const SCE_BT_ERROR_AVCTP_SEND_BUSY: Type = 2150565381;
    pub const SCE_BT_ERROR_AVCTP_SEND_NO_PRESS: Type = 2150565382;
    pub const SCE_BT_ERROR_AVCTP_SEND_NO_RELEASE: Type = 2150565383;
    pub const SCE_BT_ERROR_AVCTP_READ_NO_VOLUME: Type = 2150565384;
    pub const SCE_BT_ERROR_AVCTP_SEND_NOT_RUBY: Type = 2150565385;
    pub const SCE_BT_ERROR_HID_OPEN_NO_L2C: Type = 2150566145;
    pub const SCE_BT_ERROR_HID_CLOSE_NO_L2C: Type = 2150566146;
    pub const SCE_BT_ERROR_HID_SEND_NO_L2C: Type = 2150566147;
    pub const SCE_BT_ERROR_HID_NOT_CONNECTED: Type = 2150566148;
    pub const SCE_BT_ERROR_HID_NO_CAP: Type = 2150566149;
    pub const SCE_BT_ERROR_HID_INVALID_REQUEST_TYPE: Type = 2150566150;
    pub const SCE_BT_ERROR_HID_INVALID_BUFFER_ADDRESS: Type = 2150566151;
    pub const SCE_BT_ERROR_HID_INVALID_PROTOCOL: Type = 2150566152;
    pub const SCE_BT_ERROR_HID_INVALID_IDLE: Type = 2150566153;
    pub const SCE_BT_ERROR_HID_NOT_YET: Type = 2150566156;
    pub const SCE_BT_ERROR_HID_INVALID_LENGTH: Type = 2150566157;
    pub const SCE_BT_ERROR_HID_INVALID_REPORT_ID: Type = 2150566158;
    pub const SCE_BT_ERROR_HID_OVERWRITE_REQ: Type = 2150566159;
    pub const SCE_BT_ERROR_JUMBO_UNLOCK_NOT_OWNER: Type = 2150566657;
    pub const SCE_BT_ERROR_HCI_TX_OVERFLOW: Type = 2150566658;
    pub const SCE_BT_ERROR_ACL_TX_BUF_OVERFLOW: Type = 2150566659;
    pub const SCE_BT_ERROR_ACL_TX_CB_OVERFLOW: Type = 2150566660;
    pub const SCE_BT_ERROR_TIMER_CANCEL_BAD_ID: Type = 2150566661;
    pub const SCE_BT_ERROR_TIMER_CANCEL_NOT_INITIALIZED: Type = 2150566662;
    pub const SCE_BT_ERROR_TIMER_SET_NOT_INITIALIZED: Type = 2150566663;
    pub const SCE_BT_ERROR_TIMER_SET_NO_SPACE: Type = 2150566664;
    pub const SCE_BT_ERROR_COPYIN_FAILED: Type = 2150566665;
    pub const SCE_BT_ERROR_COPYOUT_FAILED: Type = 2150566666;
    pub const SCE_BT_ERROR_GET_NAME_NO_DEVICE: Type = 2150566913;
    pub const SCE_BT_ERROR_CB_TOO_MANY: Type = 2150566914;
    pub const SCE_BT_ERROR_CB_NOT_REGISTERED: Type = 2150566915;
    pub const SCE_BT_ERROR_CB_OVERFLOW: Type = 2150566916;
    pub const SCE_BT_ERROR_AUDIO_START_NOT_CONNECTED: Type = 2150567169;
    pub const SCE_BT_ERROR_AUDIO_START_NO_CAP: Type = 2150567170;
    pub const SCE_BT_ERROR_AUDIO_STOP_NOT_CONNECTED: Type = 2150567171;
    pub const SCE_BT_ERROR_AUDIO_STOP_NO_CAP: Type = 2150567172;
    pub const SCE_BT_ERROR_AUDIO_SEND_NOT_CONNECTED: Type = 2150567173;
    pub const SCE_BT_ERROR_AUDIO_SEND_NOT_STARTED: Type = 2150567174;
    pub const SCE_BT_ERROR_AUDIO_SEND_INVALID_LENGTH: Type = 2150567175;
    pub const SCE_BT_ERROR_AUDIO_RECV_NOT_CONNECTED: Type = 2150567176;
    pub const SCE_BT_ERROR_AUDIO_RECV_NOT_STARTED: Type = 2150567177;
    pub const SCE_BT_ERROR_AUDIO_RECV_INVALID_LENGTH: Type = 2150567178;
    pub const SCE_BT_ERROR_AUDIO_START_INVALID_SERV: Type = 2150567179;
    pub const SCE_BT_ERROR_AUDIO_STOP_INVALID_SERV: Type = 2150567180;
    pub const SCE_BT_ERROR_AUDIO_START_SERV_FAILED: Type = 2150567181;
    pub const SCE_BT_ERROR_AUDIO_STOP_SERV_FAILED: Type = 2150567182;
    pub const SCE_BT_ERROR_AUDIO_FREQ_NOT_CONNECTED: Type = 2150567183;
    pub const SCE_BT_ERROR_AUDIO_SEND_NO_CP: Type = 2150567184;
    pub const SCE_BT_ERROR_AUDIO_SEND_BAD_TYPE: Type = 2150567185;
    pub const SCE_BT_ERROR_AUDIO_RECV_BAD_TYPE: Type = 2150567186;
    pub const SCE_BT_ERROR_AUDIO_COMBI_NOT_FOUND: Type = 2150567190;
    pub const SCE_BT_ERROR_AUDIO_SEND_NO_L2C: Type = 2150567191;
    pub const SCE_BT_ERROR_AUDIO_INTERNAL_1: Type = 2150567192;
    pub const SCE_BT_ERROR_AUDIO_SEND_BUSY: Type = 2150567193;
    pub const SCE_BT_ERROR_AUDIO_RECV_BUSY: Type = 2150567194;
    pub const SCE_BT_ERROR_AUDIO_SEND_NO_CAP: Type = 2150567195;
    pub const SCE_BT_ERROR_AUDIO_SEND_SERV_FAILED: Type = 2150567196;
    pub const SCE_BT_ERROR_AUDIO_RECV_NO_CAP: Type = 2150567197;
    pub const SCE_BT_ERROR_AUDIO_RECV_SERV_FAILED: Type = 2150567198;
    pub const SCE_BT_ERROR_HID_RECV_NOT_CONNECTED: Type = 2150567425;
    pub const SCE_BT_ERROR_HID_RECV_INVALID_LENGTH: Type = 2150567426;
    pub const SCE_BT_ERROR_AVRCP_TOO_LONG_TITLE: Type = 2150568449;
    pub const SCE_BT_ERROR_AVRCP_INVALID_PLAY_STATUS: Type = 2150568450;
    pub const SCE_BT_ERROR_CONF_NOT_READY: Type = 2150569217;
    pub const SCE_BT_ERROR_CONF_INVALID_VALUE: Type = 2150569218;
    pub const SCE_BT_ERROR_CONF_BT_INACTIVE: Type = 2150569219;
    pub const SCE_BT_ERROR_CONF_TIMEOUT: Type = 2150569220;
    pub const SCE_BT_ERROR_CONF_CARD_NOT_FOUND: Type = 2150569221;
    pub const SCE_BT_ERROR_CONF_FUNCTION_NOT_FOUND: Type = 2150569222;
    pub const SCE_BT_ERROR_CONF_CANT_ENABLE_FUNCTION: Type = 2150569223;
    pub const SCE_BT_ERROR_CONF_CANT_DISABLE_FUNCTION: Type = 2150569224;
    pub const SCE_BT_ERROR_CONF_REGISTER_SUBINTR_HANDLER: Type = 2150569225;
    pub const SCE_BT_ERROR_CONF_RELEASE_SUBINTR_HANDLER: Type = 2150569226;
    pub const SCE_BT_ERROR_CONF_ENABLE_SUBINTR: Type = 2150569227;
    pub const SCE_BT_ERROR_CONF_DISABLE_SUBINTR: Type = 2150569228;
    pub const SCE_BT_ERROR_CONF_ON_TIMEOUT: Type = 2150569229;
    pub const SCE_BT_ERROR_CONF_OFF_TIMEOUT: Type = 2150569230;
    pub const SCE_BT_ERROR_CONF_SUSPEND_TIMEOUT: Type = 2150569231;
    pub const SCE_BT_ERROR_CONF_CANT_ENTER: Type = 2150569248;
    pub const SCE_BT_ERROR_NOTIMP: Type = 2150572033;
    pub const SCE_BT_ERROR_KPROC_CREATE: Type = 2150572034;
    pub const SCE_BT_ERROR_SDIO_GET_FUNCTION: Type = 2150572035;
    pub const SCE_BT_ERROR_SDIO_REGISTER_INTR_HANDLER: Type = 2150572036;
    pub const SCE_BT_ERROR_SDIO_UNREGISTER_INTR_HANDLER: Type = 2150572037;
    pub const SCE_BT_ERROR_SDIO_LOCK: Type = 2150572038;
    pub const SCE_BT_ERROR_SDIO_UNLOCK: Type = 2150572039;
    pub const SCE_BT_ERROR_SDIO_ENABLE_FUNCTION: Type = 2150572040;
    pub const SCE_BT_ERROR_SDIO_DISABLE_FUNCTION: Type = 2150572041;
    pub const SCE_BT_ERROR_SDIO_SET_BLOCK_LEN: Type = 2150572042;
    pub const SCE_BT_ERROR_SDIO_SET_BUS_SPEED: Type = 2150572043;
    pub const SCE_BT_ERROR_SDIO_READ_DIR: Type = 2150572044;
    pub const SCE_BT_ERROR_SDIO_WRITE_DIR: Type = 2150572045;
    pub const SCE_BT_ERROR_SDIO_READ_FIX: Type = 2150572046;
    pub const SCE_BT_ERROR_SDIO_WRITE_FIX: Type = 2150572047;
    pub const SCE_BT_ERROR_TSLEEP: Type = 2150572048;
    pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_REQUEST: Type = 2150572289;
    pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_REQUEST: Type = 2150572290;
    pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_SIZE: Type = 2150572291;
    pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_SIZE: Type = 2150572292;
    pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_ARGUMENT: Type = 2150572293;
    pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_ARGUMENT: Type = 2150572294;
    pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_STATE: Type = 2150572295;
    pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_STATE: Type = 2150572296;
    pub const SCE_BT_ERROR_GET_DEBUG_INFO_NOT_CONNECTED: Type = 2150572297;
    pub const SCE_BT_ERROR_SET_DEBUG_INFO_NOT_CONNECTED: Type = 2150572298;
    pub const SCE_BT_ERROR_PIN_IS_LE: Type = 2150572545;
    pub const SCE_BT_ERROR_CONNECT_START_IS_LE: Type = 2150572546;
    pub const SCE_BT_ERROR_AVCTP_IS_LE: Type = 2150572547;
    pub const SCE_BT_ERROR_HID_IS_LE: Type = 2150572548;
    pub const SCE_BT_ERROR_AUDIO_START_IS_LE: Type = 2150572549;
    pub const SCE_BT_ERROR_AUDIO_STOP_IS_LE: Type = 2150572550;
    pub const SCE_BT_ERROR_AUDIO_SEND_IS_LE: Type = 2150572551;
    pub const SCE_BT_ERROR_AUDIO_RECV_IS_LE: Type = 2150572552;
    pub const SCE_BT_ERROR_AUDIO_FREQ_IS_LE: Type = 2150572553;
    pub const SCE_BT_ERROR_ATT_BASE: Type = 2150572800;
    pub const SCE_BT_ERROR_ATT_INVALID_HANDLE: Type = 2150572801;
    pub const SCE_BT_ERROR_ATT_READ_NOT_PERMITTED: Type = 2150572802;
    pub const SCE_BT_ERROR_ATT_WRITE_NOT_PERMITTED: Type = 2150572803;
    pub const SCE_BT_ERROR_ATT_INVALID_PDU: Type = 2150572804;
    pub const SCE_BT_ERROR_ATT_INSUFFICIENT_AUTHENTICATION: Type = 2150572805;
    pub const SCE_BT_ERROR_ATT_REQUEST_NOT_SUPPORTED: Type = 2150572806;
    pub const SCE_BT_ERROR_ATT_INVALID_OFFSET: Type = 2150572807;
    pub const SCE_BT_ERROR_ATT_INSUFFICIENT_AUTHORIZATION: Type = 2150572808;
    pub const SCE_BT_ERROR_ATT_PEPARE_QUEUE_FULL: Type = 2150572809;
    pub const SCE_BT_ERROR_ATT_ATTRIBUTE_NOT_FOUND: Type = 2150572810;
    pub const SCE_BT_ERROR_ATT_ATTRIBUTE_NOT_LONG: Type = 2150572811;
    pub const SCE_BT_ERROR_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: Type = 2150572812;
    pub const SCE_BT_ERROR_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: Type = 2150572813;
    pub const SCE_BT_ERROR_ATT_UNLIKELY_ERROR: Type = 2150572814;
    pub const SCE_BT_ERROR_ATT_INSUFFICIENT_ENCRYPTION: Type = 2150572815;
    pub const SCE_BT_ERROR_ATT_UNSUPPORTED_GROUP_TYPE: Type = 2150572816;
    pub const SCE_BT_ERROR_ATT_INSUFFICIENT_RESOURCES: Type = 2150572817;
    pub const SCE_BT_ERROR_ATT_APPLICATION_ERROR_LO: Type = 2150572928;
    pub const SCE_BT_ERROR_ATT_APPLICATION_ERROR_HI: Type = 2150573055;
    pub const SCE_BT_ERROR_GATT_INVALID_NO: Type = 2150573057;
    pub const SCE_BT_ERROR_GATT_NOT_CONNECTED: Type = 2150573058;
    pub const SCE_BT_ERROR_GATT_BUSY: Type = 2150573059;
    pub const SCE_BT_ERROR_GATT_INVALID_HANDLE: Type = 2150573060;
    pub const SCE_BT_ERROR_GATT_INVALID_FLAGS: Type = 2150573061;
    pub const SCE_BT_ERROR_GATT_INVALID_SIZE: Type = 2150573062;
    pub const SCE_BT_ERROR_GATT_NOT_LE: Type = 2150573063;
    pub const SCE_BT_ERROR_GATT_TOO_BIG_RECORD: Type = 2150573064;
    pub const SCE_BT_ERROR_GATT_NOT_YET: Type = 2150573065;
    pub const SCE_BT_ERROR_GATT_DISCONNECT: Type = 2150573066;
    pub const SCE_BT_ERROR_GATT_TSLEEP: Type = 2150573067;
    pub const SCE_BT_ERROR_GATT_ENTER: Type = 2150573068;
    pub const SCE_BT_ERROR_GATT_TOO_BIG_BUFFER: Type = 2150573069;
    pub const SCE_BT_ERROR_SM_INVALID_KEY_LENGTH: Type = 2150573313;
    pub const SCE_BT_ERROR_SM_NOT_DIGIT: Type = 2150573314;
    pub const SCE_BT_ERROR_SM_NO_REQ: Type = 2150573315;
    pub const SCE_BT_ERROR_SM_NOT_NEEDED_PIN: Type = 2150573316;
    pub const SCE_BT_ERROR_SM_INVALID_CONFIRM_REPLY: Type = 2150573317;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_REQ: Type = 2150573569;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_LENGTH: Type = 2150573570;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_INTERVAL_MIN: Type = 2150573571;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_INTERVAL_MAX: Type = 2150573572;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_OWN_ADDRESS_TYPE: Type = 2150573573;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_DIRECT_ADDRESS_TYPE: Type = 2150573574;
    pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_DIRECT_ADDRESS: Type = 2150573575;
    pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_REQ: Type = 2150573825;
    pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_LENGTH: Type = 2150573826;
    pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_INTERVAL: Type = 2150573827;
    pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_WINDOW: Type = 2150573828;
    pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_OWN_ADDRESS_TYPE: Type = 2150573829;
    pub const SCE_BT_ERROR_LE_GET_ADVERTISING_NOT_FOUND: Type = 2150574081;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_SCAN_INTERVAL: Type = 2150574337;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_SCAN_WINDOW: Type = 2150574338;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_PEER_ADDRESS: Type = 2150574339;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_OWN_ADDRESS_TYPE: Type = 2150574340;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_INTERVAL_MIN: Type = 2150574341;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_INTERVAL_MAX: Type = 2150574342;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_LATENCY: Type = 2150574343;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_TIMEOUT: Type = 2150574344;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_REG_FULL: Type = 2150574345;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_REG_ERROR: Type = 2150574346;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_CONNECTED: Type = 2150574347;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_FULL: Type = 2150574348;
    pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_BUSY: Type = 2150574349;
    pub const SCE_BT_ERROR_ATT_READ_INVALID_NO: Type = 2150574593;
    pub const SCE_BT_ERROR_ATT_READ_INVALID_LENGTH: Type = 2150574594;
    pub const SCE_BT_ERROR_ATT_READ_INVALID_INTERNAL: Type = 2150574595;
    pub const SCE_BT_ERROR_ATT_WRITE_INVALID_NO: Type = 2150574596;
    pub const SCE_BT_ERROR_ATT_WRITE_INVALID_LENGTH: Type = 2150574597;
    pub const SCE_BT_ERROR_ATT_WRITE_INVALID_INTERNAL: Type = 2150574598;
    pub const SCE_BT_ERROR_ATT_NOT_YET: Type = 2150574599;
    pub const SCE_BT_ERROR_LE_NOT_SUPPORTED: Type = 2150574849;
    pub const SCE_BT_ERROR_PAIRING_OOB_TIMEOUT: Type = 2150576385;
    pub const SCE_BT_ERROR_PAIRING_OOB_FULL: Type = 2150576386;
    pub const SCE_BT_ERROR_PAIRING_OOB_CAN_NOT_DISCONNECT: Type = 2150576387;
    pub const SCE_BT_ERROR_PAIRING_OOB_INTERNAL_ERROR: Type = 2150576388;
    pub const SCE_BT_ERROR_NOT_READY: Type = 2150576641;
    pub const SCE_BT_ERROR_GET_JACK_STATUS_NOT_CONNECTED: Type = 2150576897;
    pub const SCE_BT_ERROR_TOO_MANY_CONNECTION: Type = 2150577153;
    pub const SCE_BT_ERROR_TOO_MANY_HID: Type = 2150577154;
    pub const SCE_BT_ERROR_NOT_SUPPORTED_DEVICE: Type = 2150577155;
    pub const SCE_BT_ERROR_JEDI_VOLUME_GAIN_NOT_CONNECTED: Type = 2150577409;
    pub const SCE_BT_ERROR_JEDI_SNIFF_NOT_CONNECTED: Type = 2150577665;
    pub const SCE_BT_ERROR_JEDI_SNIFF_NOT_JEDI: Type = 2150577666;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceBtRegisteredInfo {
    pub mac: [crate::ctypes::c_uchar; 6usize],
    pub unk0: crate::ctypes::c_ushort,
    pub bt_class: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub vid: crate::ctypes::c_ushort,
    pub pid: crate::ctypes::c_ushort,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub name: [crate::ctypes::c_char; 128usize],
    pub unk5: [crate::ctypes::c_uchar; 96usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceBtEvent {
    pub __bindgen_anon_1: SceBtEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceBtEvent__bindgen_ty_1 {
    pub data: [crate::ctypes::c_uchar; 16usize],
    pub __bindgen_anon_1: SceBtEvent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceBtEvent__bindgen_ty_1__bindgen_ty_1 {
    pub id: crate::ctypes::c_uchar,
    pub unk1: crate::ctypes::c_uchar,
    pub unk2: crate::ctypes::c_ushort,
    pub unk3: crate::ctypes::c_uint,
    pub mac0: crate::ctypes::c_uint,
    pub mac1: crate::ctypes::c_uint,
}
pub type SceBtCallback = ::core::option::Option<
    unsafe extern "C" fn(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ),
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _SceBtHidRequest {
    pub unk00: u32,
    pub unk04: u32,
    pub type_: u8,
    pub unk09: u8,
    pub unk0A: u8,
    pub unk0B: u8,
    pub buffer: *mut crate::ctypes::c_void,
    pub length: u32,
    pub next: *mut _SceBtHidRequest,
}
pub type SceBtHidRequest = _SceBtHidRequest;
extern "C" {
    pub fn ksceBtAvrcpReadVolume(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtAvrcpSendButton(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtAvrcpSendVolume(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtAvrcpSetPlayStatus(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtAvrcpSetTitle(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtDeleteRegisteredInfo(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtFreqAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetConfiguration() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetConnectingInfo(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetDeviceName(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        name: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetInfoForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetLastError() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetRegisteredInfo(
        device: crate::ctypes::c_int,
        unk: crate::ctypes::c_int,
        info: *mut SceBtRegisteredInfo,
        info_size: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetStatusForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtGetVidPid(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        vid_pid: *mut crate::ctypes::c_ushort,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtHfpGetCurrentPhoneNumber(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtHfpRequest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtHidGetReportDescriptor(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        buffer: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtHidTransfer(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        request: *mut SceBtHidRequest,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtPushBip(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtPushOpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtReadEvent(
        events: *mut SceBtEvent,
        num_events: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtRecvAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtRecvBip(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtRecvOpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtRecvSpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtRegisterCallback(
        cb: SceUID,
        unused: crate::ctypes::c_int,
        flags1: crate::ctypes::c_int,
        flags2: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtReplyPinCode(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        code: *mut crate::ctypes::c_uchar,
        length: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtReplyUserConfirmation(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSendAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSendL2capEchoRequestForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSendSpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSetConfiguration(r0: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSetContentProtection(r0: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSetInquiryResultForTest(arg1: *mut crate::ctypes::c_uchar)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSetInquiryScan(r0: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSetL2capEchoResponseBufferForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtSetStatusForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtStartAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtStartConnect(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtStartDisconnect(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtStartInquiry() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtStopAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtStopInquiry() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceBtUnregisterCallback(cb: SceUID) -> crate::ctypes::c_int;
}
