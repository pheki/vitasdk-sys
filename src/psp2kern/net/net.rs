/* automatically generated by rust-bindgen 0.65.1 */

pub const SCE_NET_IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const SCE_NET_IP_DEFAULT_MULTICAST_LOOP: u32 = 1;
pub const SCE_NET_IPVERSION: u32 = 4;
pub const SCE_NET_IP_RF: u32 = 32768;
pub const SCE_NET_IP_DF: u32 = 16384;
pub const SCE_NET_IP_MF: u32 = 8192;
pub const SCE_NET_IP_OFFMASK: u32 = 8191;
pub const SCE_NET_INADDR_ANY: u32 = 0;
pub const SCE_NET_INADDR_LOOPBACK: u32 = 2130706433;
pub const SCE_NET_INADDR_BROADCAST: u32 = 4294967295;
pub const SCE_NET_INADDR_UNSPEC_GROUP: u32 = 3758096384;
pub const SCE_NET_INADDR_AUTOIP: u32 = 2851995648;
pub const SCE_NET_IN_CLASSD_NET: u32 = 4026531840;
pub const SCE_NET_IN_AUTOIP_NET: u32 = 4294901760;
pub const SCE_NET_ADHOC_PORT: u32 = 3658;
pub const SCE_NET_AF_INET: u32 = 2;
pub const SCE_NET_DEBUG_NAME_LEN_MAX: u32 = 31;
pub const SCE_NET_ID_SOCKET_MIN: u32 = 0;
pub const SCE_NET_ID_SOCKET_MAX: u32 = 1023;
pub mod SceNetErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_ERROR_EPERM: Type = 2151743745;
    pub const SCE_NET_ERROR_ENOENT: Type = 2151743746;
    pub const SCE_NET_ERROR_ESRCH: Type = 2151743747;
    pub const SCE_NET_ERROR_EINTR: Type = 2151743748;
    pub const SCE_NET_ERROR_EIO: Type = 2151743749;
    pub const SCE_NET_ERROR_ENXIO: Type = 2151743750;
    pub const SCE_NET_ERROR_E2BIG: Type = 2151743751;
    pub const SCE_NET_ERROR_ENOEXEC: Type = 2151743752;
    pub const SCE_NET_ERROR_EBADF: Type = 2151743753;
    pub const SCE_NET_ERROR_ECHILD: Type = 2151743754;
    pub const SCE_NET_ERROR_EDEADLK: Type = 2151743755;
    pub const SCE_NET_ERROR_ENOMEM: Type = 2151743756;
    pub const SCE_NET_ERROR_EACCES: Type = 2151743757;
    pub const SCE_NET_ERROR_EFAULT: Type = 2151743758;
    pub const SCE_NET_ERROR_ENOTBLK: Type = 2151743759;
    pub const SCE_NET_ERROR_EBUSY: Type = 2151743760;
    pub const SCE_NET_ERROR_EEXIST: Type = 2151743761;
    pub const SCE_NET_ERROR_EXDEV: Type = 2151743762;
    pub const SCE_NET_ERROR_ENODEV: Type = 2151743763;
    pub const SCE_NET_ERROR_ENOTDIR: Type = 2151743764;
    pub const SCE_NET_ERROR_EISDIR: Type = 2151743765;
    pub const SCE_NET_ERROR_EINVAL: Type = 2151743766;
    pub const SCE_NET_ERROR_ENFILE: Type = 2151743767;
    pub const SCE_NET_ERROR_EMFILE: Type = 2151743768;
    pub const SCE_NET_ERROR_ENOTTY: Type = 2151743769;
    pub const SCE_NET_ERROR_ETXTBSY: Type = 2151743770;
    pub const SCE_NET_ERROR_EFBIG: Type = 2151743771;
    pub const SCE_NET_ERROR_ENOSPC: Type = 2151743772;
    pub const SCE_NET_ERROR_ESPIPE: Type = 2151743773;
    pub const SCE_NET_ERROR_EROFS: Type = 2151743774;
    pub const SCE_NET_ERROR_EMLINK: Type = 2151743775;
    pub const SCE_NET_ERROR_EPIPE: Type = 2151743776;
    pub const SCE_NET_ERROR_EDOM: Type = 2151743777;
    pub const SCE_NET_ERROR_ERANGE: Type = 2151743778;
    pub const SCE_NET_ERROR_EAGAIN: Type = 2151743779;
    pub const SCE_NET_ERROR_EWOULDBLOCK: Type = 2151743779;
    pub const SCE_NET_ERROR_EINPROGRESS: Type = 2151743780;
    pub const SCE_NET_ERROR_EALREADY: Type = 2151743781;
    pub const SCE_NET_ERROR_ENOTSOCK: Type = 2151743782;
    pub const SCE_NET_ERROR_EDESTADDRREQ: Type = 2151743783;
    pub const SCE_NET_ERROR_EMSGSIZE: Type = 2151743784;
    pub const SCE_NET_ERROR_EPROTOTYPE: Type = 2151743785;
    pub const SCE_NET_ERROR_ENOPROTOOPT: Type = 2151743786;
    pub const SCE_NET_ERROR_EPROTONOSUPPORT: Type = 2151743787;
    pub const SCE_NET_ERROR_ESOCKTNOSUPPORT: Type = 2151743788;
    pub const SCE_NET_ERROR_EOPNOTSUPP: Type = 2151743789;
    pub const SCE_NET_ERROR_EPFNOSUPPORT: Type = 2151743790;
    pub const SCE_NET_ERROR_EAFNOSUPPORT: Type = 2151743791;
    pub const SCE_NET_ERROR_EADDRINUSE: Type = 2151743792;
    pub const SCE_NET_ERROR_EADDRNOTAVAIL: Type = 2151743793;
    pub const SCE_NET_ERROR_ENETDOWN: Type = 2151743794;
    pub const SCE_NET_ERROR_ENETUNREACH: Type = 2151743795;
    pub const SCE_NET_ERROR_ENETRESET: Type = 2151743796;
    pub const SCE_NET_ERROR_ECONNABORTED: Type = 2151743797;
    pub const SCE_NET_ERROR_ECONNRESET: Type = 2151743798;
    pub const SCE_NET_ERROR_ENOBUFS: Type = 2151743799;
    pub const SCE_NET_ERROR_EISCONN: Type = 2151743800;
    pub const SCE_NET_ERROR_ENOTCONN: Type = 2151743801;
    pub const SCE_NET_ERROR_ESHUTDOWN: Type = 2151743802;
    pub const SCE_NET_ERROR_ETOOMANYREFS: Type = 2151743803;
    pub const SCE_NET_ERROR_ETIMEDOUT: Type = 2151743804;
    pub const SCE_NET_ERROR_ECONNREFUSED: Type = 2151743805;
    pub const SCE_NET_ERROR_ELOOP: Type = 2151743806;
    pub const SCE_NET_ERROR_ENAMETOOLONG: Type = 2151743807;
    pub const SCE_NET_ERROR_EHOSTDOWN: Type = 2151743808;
    pub const SCE_NET_ERROR_EHOSTUNREACH: Type = 2151743809;
    pub const SCE_NET_ERROR_ENOTEMPTY: Type = 2151743810;
    pub const SCE_NET_ERROR_EPROCLIM: Type = 2151743811;
    pub const SCE_NET_ERROR_EUSERS: Type = 2151743812;
    pub const SCE_NET_ERROR_EDQUOT: Type = 2151743813;
    pub const SCE_NET_ERROR_ESTALE: Type = 2151743814;
    pub const SCE_NET_ERROR_EREMOTE: Type = 2151743815;
    pub const SCE_NET_ERROR_EBADRPC: Type = 2151743816;
    pub const SCE_NET_ERROR_ERPCMISMATCH: Type = 2151743817;
    pub const SCE_NET_ERROR_EPROGUNAVAIL: Type = 2151743818;
    pub const SCE_NET_ERROR_EPROGMISMATCH: Type = 2151743819;
    pub const SCE_NET_ERROR_EPROCUNAVAIL: Type = 2151743820;
    pub const SCE_NET_ERROR_ENOLCK: Type = 2151743821;
    pub const SCE_NET_ERROR_ENOSYS: Type = 2151743822;
    pub const SCE_NET_ERROR_EFTYPE: Type = 2151743823;
    pub const SCE_NET_ERROR_EAUTH: Type = 2151743824;
    pub const SCE_NET_ERROR_ENEEDAUTH: Type = 2151743825;
    pub const SCE_NET_ERROR_EIDRM: Type = 2151743826;
    pub const SCE_NET_ERROR_ENOMS: Type = 2151743827;
    pub const SCE_NET_ERROR_EOVERFLOW: Type = 2151743828;
    pub const SCE_NET_ERROR_EILSEQ: Type = 2151743829;
    pub const SCE_NET_ERROR_ENOTSUP: Type = 2151743830;
    pub const SCE_NET_ERROR_ECANCELED: Type = 2151743831;
    pub const SCE_NET_ERROR_EBADMSG: Type = 2151743832;
    pub const SCE_NET_ERROR_ENODATA: Type = 2151743833;
    pub const SCE_NET_ERROR_ENOSR: Type = 2151743834;
    pub const SCE_NET_ERROR_ENOSTR: Type = 2151743835;
    pub const SCE_NET_ERROR_ETIME: Type = 2151743836;
    pub const SCE_NET_ERROR_EADHOC: Type = 2151743904;
    pub const SCE_NET_ERROR_EDISABLEDIF: Type = 2151743905;
    pub const SCE_NET_ERROR_ERESUME: Type = 2151743906;
    pub const SCE_NET_ERROR_ENOTINIT: Type = 2151743944;
    pub const SCE_NET_ERROR_ENOLIBMEM: Type = 2151743945;
    pub const SCE_NET_ERROR_ERESERVED202: Type = 2151743946;
    pub const SCE_NET_ERROR_ECALLBACK: Type = 2151743947;
    pub const SCE_NET_ERROR_EINTERNAL: Type = 2151743948;
    pub const SCE_NET_ERROR_ERETURN: Type = 2151743949;
    pub const SCE_NET_ERROR_RESOLVER_EINTERNAL: Type = 2151743964;
    pub const SCE_NET_ERROR_RESOLVER_EBUSY: Type = 2151743965;
    pub const SCE_NET_ERROR_RESOLVER_ENOSPACE: Type = 2151743966;
    pub const SCE_NET_ERROR_RESOLVER_EPACKET: Type = 2151743967;
    pub const SCE_NET_ERROR_RESOLVER_ERESERVED22: Type = 2151743968;
    pub const SCE_NET_ERROR_RESOLVER_ENODNS: Type = 2151743969;
    pub const SCE_NET_ERROR_RESOLVER_ETIMEDOUT: Type = 2151743970;
    pub const SCE_NET_ERROR_RESOLVER_ENOSUPPORT: Type = 2151743971;
    pub const SCE_NET_ERROR_RESOLVER_EFORMAT: Type = 2151743972;
    pub const SCE_NET_ERROR_RESOLVER_ESERVERFAILURE: Type = 2151743973;
    pub const SCE_NET_ERROR_RESOLVER_ENOHOST: Type = 2151743974;
    pub const SCE_NET_ERROR_RESOLVER_ENOTIMPLEMENTED: Type = 2151743975;
    pub const SCE_NET_ERROR_RESOLVER_ESERVERREFUSED: Type = 2151743976;
    pub const SCE_NET_ERROR_RESOLVER_ENORECORD: Type = 2151743977;
    pub const SCE_NET_ERROR_RESOLVER_EALIGNMENT: Type = 2151743978;
}
pub mod SceNetKernelErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_EPERM: Type = 1;
    pub const SCE_NET_ENOENT: Type = 2;
    pub const SCE_NET_ESRCH: Type = 3;
    pub const SCE_NET_EINTR: Type = 4;
    pub const SCE_NET_EIO: Type = 5;
    pub const SCE_NET_ENXIO: Type = 6;
    pub const SCE_NET_E2BIG: Type = 7;
    pub const SCE_NET_ENOEXEC: Type = 8;
    pub const SCE_NET_EBADF: Type = 9;
    pub const SCE_NET_ECHILD: Type = 10;
    pub const SCE_NET_EDEADLK: Type = 11;
    pub const SCE_NET_ENOMEM: Type = 12;
    pub const SCE_NET_EACCES: Type = 13;
    pub const SCE_NET_EFAULT: Type = 14;
    pub const SCE_NET_ENOTBLK: Type = 15;
    pub const SCE_NET_EBUSY: Type = 16;
    pub const SCE_NET_EEXIST: Type = 17;
    pub const SCE_NET_EXDEV: Type = 18;
    pub const SCE_NET_ENODEV: Type = 19;
    pub const SCE_NET_ENOTDIR: Type = 20;
    pub const SCE_NET_EISDIR: Type = 21;
    pub const SCE_NET_EINVAL: Type = 22;
    pub const SCE_NET_ENFILE: Type = 23;
    pub const SCE_NET_EMFILE: Type = 24;
    pub const SCE_NET_ENOTTY: Type = 25;
    pub const SCE_NET_ETXTBSY: Type = 26;
    pub const SCE_NET_EFBIG: Type = 27;
    pub const SCE_NET_ENOSPC: Type = 28;
    pub const SCE_NET_ESPIPE: Type = 29;
    pub const SCE_NET_EROFS: Type = 30;
    pub const SCE_NET_EMLINK: Type = 31;
    pub const SCE_NET_EPIPE: Type = 32;
    pub const SCE_NET_EDOM: Type = 33;
    pub const SCE_NET_ERANGE: Type = 34;
    pub const SCE_NET_EAGAIN: Type = 35;
    pub const SCE_NET_EWOULDBLOCK: Type = 35;
    pub const SCE_NET_EINPROGRESS: Type = 36;
    pub const SCE_NET_EALREADY: Type = 37;
    pub const SCE_NET_ENOTSOCK: Type = 38;
    pub const SCE_NET_EDESTADDRREQ: Type = 39;
    pub const SCE_NET_EMSGSIZE: Type = 40;
    pub const SCE_NET_EPROTOTYPE: Type = 41;
    pub const SCE_NET_ENOPROTOOPT: Type = 42;
    pub const SCE_NET_EPROTONOSUPPORT: Type = 43;
    pub const SCE_NET_ESOCKTNOSUPPORT: Type = 44;
    pub const SCE_NET_EOPNOTSUPP: Type = 45;
    pub const SCE_NET_EPFNOSUPPORT: Type = 46;
    pub const SCE_NET_EAFNOSUPPORT: Type = 47;
    pub const SCE_NET_EADDRINUSE: Type = 48;
    pub const SCE_NET_EADDRNOTAVAIL: Type = 49;
    pub const SCE_NET_ENETDOWN: Type = 50;
    pub const SCE_NET_ENETUNREACH: Type = 51;
    pub const SCE_NET_ENETRESET: Type = 52;
    pub const SCE_NET_ECONNABORTED: Type = 53;
    pub const SCE_NET_ECONNRESET: Type = 54;
    pub const SCE_NET_ENOBUFS: Type = 55;
    pub const SCE_NET_EISCONN: Type = 56;
    pub const SCE_NET_ENOTCONN: Type = 57;
    pub const SCE_NET_ESHUTDOWN: Type = 58;
    pub const SCE_NET_ETOOMANYREFS: Type = 59;
    pub const SCE_NET_ETIMEDOUT: Type = 60;
    pub const SCE_NET_ECONNREFUSED: Type = 61;
    pub const SCE_NET_ELOOP: Type = 62;
    pub const SCE_NET_ENAMETOOLONG: Type = 63;
    pub const SCE_NET_EHOSTDOWN: Type = 64;
    pub const SCE_NET_EHOSTUNREACH: Type = 65;
    pub const SCE_NET_ENOTEMPTY: Type = 66;
    pub const SCE_NET_EPROCLIM: Type = 67;
    pub const SCE_NET_EUSERS: Type = 68;
    pub const SCE_NET_EDQUOT: Type = 69;
    pub const SCE_NET_ESTALE: Type = 70;
    pub const SCE_NET_EREMOTE: Type = 71;
    pub const SCE_NET_EBADRPC: Type = 72;
    pub const SCE_NET_ERPCMISMATCH: Type = 73;
    pub const SCE_NET_EPROGUNAVAIL: Type = 74;
    pub const SCE_NET_EPROGMISMATCH: Type = 75;
    pub const SCE_NET_EPROCUNAVAIL: Type = 76;
    pub const SCE_NET_ENOLCK: Type = 77;
    pub const SCE_NET_ENOSYS: Type = 78;
    pub const SCE_NET_EFTYPE: Type = 79;
    pub const SCE_NET_EAUTH: Type = 80;
    pub const SCE_NET_ENEEDAUTH: Type = 81;
    pub const SCE_NET_EIDRM: Type = 82;
    pub const SCE_NET_ENOMSG: Type = 83;
    pub const SCE_NET_EOVERFLOW: Type = 84;
    pub const SCE_NET_EILSEQ: Type = 85;
    pub const SCE_NET_ENOTSUP: Type = 86;
    pub const SCE_NET_ECANCELED: Type = 87;
    pub const SCE_NET_EBADMSG: Type = 88;
    pub const SCE_NET_ENODATA: Type = 89;
    pub const SCE_NET_ENOSR: Type = 90;
    pub const SCE_NET_ENOSTR: Type = 91;
    pub const SCE_NET_ETIME: Type = 92;
    pub const SCE_NET_EADHOC: Type = 160;
    pub const SCE_NET_EDISABLEDIF: Type = 161;
    pub const SCE_NET_ERESUME: Type = 162;
}
pub mod SceNetLibnetErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_ENOTINIT: Type = 200;
    pub const SCE_NET_ENOLIBMEM: Type = 201;
    pub const SCE_NET_ETLS: Type = 202;
    pub const SCE_NET_ECALLBACK: Type = 203;
    pub const SCE_NET_EINTERNAL: Type = 204;
    pub const SCE_NET_ERETURN: Type = 205;
}
pub mod SceNetSockInfoState {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_SOCKINFO_STATE_UNKNOWN: Type = 0;
    pub const SCE_NET_SOCKINFO_STATE_CLOSED: Type = 1;
    pub const SCE_NET_SOCKINFO_STATE_OPENED: Type = 2;
    pub const SCE_NET_SOCKINFO_STATE_LISTEN: Type = 3;
    pub const SCE_NET_SOCKINFO_STATE_SYN_SENT: Type = 4;
    pub const SCE_NET_SOCKINFO_STATE_SYN_RECEIVED: Type = 5;
    pub const SCE_NET_SOCKINFO_STATE_ESTABLISHED: Type = 6;
    pub const SCE_NET_SOCKINFO_STATE_FIN_WAIT_1: Type = 7;
    pub const SCE_NET_SOCKINFO_STATE_FIN_WAIT_2: Type = 8;
    pub const SCE_NET_SOCKINFO_STATE_CLOSE_WAIT: Type = 9;
    pub const SCE_NET_SOCKINFO_STATE_CLOSING: Type = 10;
    pub const SCE_NET_SOCKINFO_STATE_LAST_ACK: Type = 11;
    pub const SCE_NET_SOCKINFO_STATE_TIME_WAIT: Type = 12;
}
pub mod SceNetSockInfoFlag {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_SOCKINFO_F_SELF: Type = 1;
    pub const SCE_NET_SOCKINFO_F_KERNEL: Type = 2;
    pub const SCE_NET_SOCKINFO_F_OTHERS: Type = 4;
    pub const SCE_NET_SOCKINFO_F_RECV_WAIT: Type = 65536;
    pub const SCE_NET_SOCKINFO_F_SEND_WAIT: Type = 131072;
    pub const SCE_NET_SOCKINFO_F_RECV_EWAIT: Type = 262144;
    pub const SCE_NET_SOCKINFO_F_SEND_EWAIT: Type = 524288;
    pub const SCE_NET_SOCKINFO_F_WAKEUP_SIGNAL: Type = 1048576;
    pub const SCE_NET_SOCKINFO_F_ALL: Type = 2031623;
}
pub mod SceNetProtocol {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_IPPROTO_IP: Type = 0;
    pub const SCE_NET_IPPROTO_ICMP: Type = 1;
    pub const SCE_NET_IPPROTO_IGMP: Type = 2;
    pub const SCE_NET_IPPROTO_TCP: Type = 6;
    pub const SCE_NET_IPPROTO_UDP: Type = 17;
    pub const SCE_NET_SOL_SOCKET: Type = 65535;
}
pub mod SceNetSocketOption {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_IP_HDRINCL: Type = 2;
    pub const SCE_NET_IP_TOS: Type = 3;
    pub const SCE_NET_IP_TTL: Type = 4;
    pub const SCE_NET_IP_MULTICAST_IF: Type = 9;
    pub const SCE_NET_IP_MULTICAST_TTL: Type = 10;
    pub const SCE_NET_IP_MULTICAST_LOOP: Type = 11;
    pub const SCE_NET_IP_ADD_MEMBERSHIP: Type = 12;
    pub const SCE_NET_IP_DROP_MEMBERSHIP: Type = 13;
    pub const SCE_NET_IP_TTLCHK: Type = 23;
    pub const SCE_NET_IP_MAXTTL: Type = 24;
    pub const SCE_NET_TCP_NODELAY: Type = 1;
    pub const SCE_NET_TCP_MAXSEG: Type = 2;
    pub const SCE_NET_TCP_MSS_TO_ADVERTISE: Type = 3;
    pub const SCE_NET_SO_REUSEADDR: Type = 4;
    pub const SCE_NET_SO_KEEPALIVE: Type = 8;
    pub const SCE_NET_SO_BROADCAST: Type = 32;
    pub const SCE_NET_SO_LINGER: Type = 128;
    pub const SCE_NET_SO_OOBINLINE: Type = 256;
    pub const SCE_NET_SO_REUSEPORT: Type = 512;
    pub const SCE_NET_SO_ONESBCAST: Type = 2048;
    pub const SCE_NET_SO_USECRYPTO: Type = 4096;
    pub const SCE_NET_SO_USESIGNATURE: Type = 8192;
    pub const SCE_NET_SO_SNDBUF: Type = 4097;
    pub const SCE_NET_SO_RCVBUF: Type = 4098;
    pub const SCE_NET_SO_SNDLOWAT: Type = 4099;
    pub const SCE_NET_SO_RCVLOWAT: Type = 4100;
    pub const SCE_NET_SO_SNDTIMEO: Type = 4101;
    pub const SCE_NET_SO_RCVTIMEO: Type = 4102;
    pub const SCE_NET_SO_ERROR: Type = 4103;
    pub const SCE_NET_SO_TYPE: Type = 4104;
    pub const SCE_NET_SO_NBIO: Type = 4352;
    pub const SCE_NET_SO_TPPOLICY: Type = 4353;
    pub const SCE_NET_SO_NAME: Type = 4354;
}
pub mod SceNetSocketType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_SOCK_STREAM: Type = 1;
    pub const SCE_NET_SOCK_DGRAM: Type = 2;
    pub const SCE_NET_SOCK_RAW: Type = 3;
    pub const SCE_NET_SOCK_DGRAM_P2P: Type = 6;
    pub const SCE_NET_SOCK_STREAM_P2P: Type = 10;
}
pub mod SceNetMsgFlag {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_MSG_PEEK: Type = 2;
    pub const SCE_NET_MSG_WAITALL: Type = 64;
    pub const SCE_NET_MSG_DONTWAIT: Type = 128;
    pub const SCE_NET_MSG_USECRYPTO: Type = 1024;
    pub const SCE_NET_MSG_USESIGNATURE: Type = 2048;
}
pub mod SceNetIcmpType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_ICMP_TYPE_ECHO_REPLY: Type = 0;
    pub const SCE_NET_ICMP_TYPE_DEST_UNREACH: Type = 3;
    pub const SCE_NET_ICMP_TYPE_SOURCE_QUENCH: Type = 4;
    pub const SCE_NET_ICMP_TYPE_REDIRECT: Type = 5;
    pub const SCE_NET_ICMP_TYPE_ECHO_REQUEST: Type = 8;
    pub const SCE_NET_ICMP_TYPE_TIME_EXCEEDED: Type = 11;
    pub const SCE_NET_ICMP_TYPE_PARAMETER_PROBLEM: Type = 12;
    pub const SCE_NET_ICMP_TYPE_TIMESTAMP_REQUEST: Type = 13;
    pub const SCE_NET_ICMP_TYPE_TIMESTAMP_REPLY: Type = 14;
    pub const SCE_NET_ICMP_TYPE_INFORMATION_REQUEST: Type = 15;
    pub const SCE_NET_ICMP_TYPE_INFORMATION_REPLY: Type = 16;
    pub const SCE_NET_ICMP_TYPE_ADDRESS_MASK_REQUEST: Type = 17;
    pub const SCE_NET_ICMP_TYPE_ADDRESS_MASK_REPLY: Type = 18;
}
pub mod SceNetIcmpCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_UNREACH: Type = 0;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_HOST_UNREACH: Type = 1;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_PROTO_UNREACH: Type = 2;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_PORT_UNREACH: Type = 3;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_FRAG_AND_DF: Type = 4;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_SRC_HOST_FAILED: Type = 5;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_DST_NET_UNKNOWN: Type = 6;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_DST_HOST_UNKNOWN: Type = 7;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_SRC_HOST_ISOLATED: Type = 8;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_ADMIN_PROHIBITED: Type = 9;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_HOST_PROHIBITED: Type = 10;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_TOS: Type = 11;
    pub const SCE_NET_ICMP_CODE_DEST_UNREACH_HOST_TOS: Type = 12;
    pub const SCE_NET_ICMP_CODE_TIME_EXCEEDED_TTL_EXCEEDED: Type = 0;
    pub const SCE_NET_ICMP_CODE_TIME_EXCEEDED_FRT_EXCEEDED: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetFdSet {
    pub bits: [crate::ctypes::c_uint; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetInAddr {
    pub s_addr: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSockaddrIn {
    pub sin_len: crate::ctypes::c_uchar,
    pub sin_family: crate::ctypes::c_uchar,
    pub sin_port: crate::ctypes::c_ushort,
    pub sin_addr: SceNetInAddr,
    pub sin_vport: crate::ctypes::c_ushort,
    pub sin_zero: [crate::ctypes::c_char; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIpMreq {
    pub imr_multiaddr: SceNetInAddr,
    pub imr_interface: SceNetInAddr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetEtherAddr {
    pub data: [crate::ctypes::c_uchar; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetDnsInfo {
    pub dns_addr: [SceNetInAddr; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetLinger {
    pub l_onoff: crate::ctypes::c_int,
    pub l_linger: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSockaddr {
    pub sa_len: crate::ctypes::c_uchar,
    pub sa_family: crate::ctypes::c_uchar,
    pub sa_data: [crate::ctypes::c_char; 14usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIovec {
    pub iov_base: *mut crate::ctypes::c_void,
    pub iov_len: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetMsghdr {
    pub msg_name: *mut crate::ctypes::c_void,
    pub msg_namelen: crate::ctypes::c_uint,
    pub msg_iov: *mut SceNetIovec,
    pub msg_iovlen: crate::ctypes::c_int,
    pub msg_control: *mut crate::ctypes::c_void,
    pub msg_controllen: crate::ctypes::c_uint,
    pub msg_flags: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSockInfo {
    pub name: [crate::ctypes::c_char; 32usize],
    pub pid: crate::ctypes::c_int,
    pub s: crate::ctypes::c_int,
    pub socket_type: crate::ctypes::c_char,
    pub policy: crate::ctypes::c_char,
    pub reserved16: crate::ctypes::c_short,
    pub recv_queue_length: crate::ctypes::c_int,
    pub send_queue_length: crate::ctypes::c_int,
    pub local_adr: SceNetInAddr,
    pub remote_adr: SceNetInAddr,
    pub local_port: crate::ctypes::c_ushort,
    pub remote_port: crate::ctypes::c_ushort,
    pub local_vport: crate::ctypes::c_ushort,
    pub remote_vport: crate::ctypes::c_ushort,
    pub state: crate::ctypes::c_int,
    pub flags: crate::ctypes::c_int,
    pub reserved: [crate::ctypes::c_int; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetStatisticsInfo {
    pub kernel_mem_free_size: crate::ctypes::c_int,
    pub kernel_mem_free_min: crate::ctypes::c_int,
    pub packet_count: crate::ctypes::c_int,
    pub packet_qos_count: crate::ctypes::c_int,
    pub libnet_mem_free_size: crate::ctypes::c_int,
    pub libnet_mem_free_min: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIpHeaderIpVerHl {
    pub hl: crate::ctypes::c_uchar,
    pub ver: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetIpHeaderUnion {
    pub ip_ver_hl: SceNetIpHeaderIpVerHl,
    pub ver_hl: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetIpHeader {
    pub un: SceNetIpHeaderUnion,
    pub ip_tos: crate::ctypes::c_uchar,
    pub ip_len: crate::ctypes::c_ushort,
    pub ip_id: crate::ctypes::c_ushort,
    pub ip_off: crate::ctypes::c_ushort,
    pub ip_ttl: crate::ctypes::c_uchar,
    pub ip_p: crate::ctypes::c_uchar,
    pub ip_sum: crate::ctypes::c_ushort,
    pub ip_src: SceNetInAddr,
    pub ip_dst: SceNetInAddr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIcmpHeaderEcho {
    pub id: crate::ctypes::c_ushort,
    pub sequence: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIcmpHeaderFrag {
    pub unused: crate::ctypes::c_ushort,
    pub mtu: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetIcmpHeaderUnion {
    pub echo: SceNetIcmpHeaderEcho,
    pub gateway: crate::ctypes::c_uint,
    pub frag: SceNetIcmpHeaderFrag,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetIcmpHeader {
    pub type_: crate::ctypes::c_uchar,
    pub code: crate::ctypes::c_uchar,
    pub checksum: crate::ctypes::c_ushort,
    pub un: SceNetIcmpHeaderUnion,
}
extern "C" {
    pub fn ksceNetSocket(
        name: *const crate::ctypes::c_char,
        domain: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        protocol: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetAccept(
        s: crate::ctypes::c_int,
        addr: *mut SceNetSockaddr,
        addrlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetBind(
        s: crate::ctypes::c_int,
        addr: *const SceNetSockaddr,
        addrlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetConnect(
        s: crate::ctypes::c_int,
        name: *const SceNetSockaddr,
        namelen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetListen(
        s: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetRecvfrom(
        s: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        from: *mut SceNetSockaddr,
        fromlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetSendto(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        to: *const SceNetSockaddr,
        tolen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetSetsockopt(
        s: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
        optname: crate::ctypes::c_int,
        optval: *const crate::ctypes::c_void,
        optlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetClose(s: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
