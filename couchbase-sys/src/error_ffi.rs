/***************
* Enumerations *
****************/
#[derive(Copy)]
pub enum lcb_error_t {
    LCB_SUCCESS = 0x00,
    LCB_AUTH_CONTINUE = 0x01,
    LCB_AUTH_ERROR = 0x02,
    LCB_DELTA_BADVAL = 0x03,
    LCB_E2BIG = 0x04,
    LCB_EBUSY = 0x05,
    LCB_EINTERNAL = 0x06,
    LCB_EINVAL = 0x07,
    LCB_ENOMEM = 0x08,
    LCB_ERANGE = 0x09,
    LCB_ERROR = 0x0A,
    LCB_ETMPFAIL = 0x0B,
    LCB_KEY_EEXISTS = 0x0C,
    LCB_KEY_ENOENT = 0x0D,
    LCB_DLOPEN_FAILED = 0x0E,
    LCB_DLSYM_FAILED = 0x0F,
    LCB_NETWORK_ERROR = 0x10,
    LCB_NOT_MY_VBUCKET = 0x11,
    LCB_NOT_STORED = 0x12,
    LCB_NOT_SUPPORTED = 0x13,
    LCB_UNKNOWN_COMMAND = 0x14,
    LCB_UNKNOWN_HOST = 0x15,
    LCB_PROTOCOL_ERROR = 0x16,
    LCB_ETIMEDOUT = 0x17,
    LCB_CONNECT_ERROR = 0x18,
    LCB_BUCKET_ENOENT = 0x19,
    LCB_CLIENT_ENOMEM = 0x1A,
    LCB_CLIENT_ETMPFAIL = 0x1B,
    LCB_EBADHANDLE = 0x1C,
    LCB_SERVER_BUG = 0x1D,
    LCB_INVALID_HOST_FORMAT = 0x1F,
    LCB_INVALID_CHAR = 0x20,
    LCB_DURABILITY_ETOOMANY = 0x21,
    LCB_DUPLICATE_COMMANDS = 0x22,
    LCB_NO_MATCHING_SERVER = 0x23,
    LCB_BAD_ENVIRONMENT = 0x24,
    LCB_BUSY = 0x25,
    LCB_INVALID_USERNAME = 0x26,
    LCB_CONFIG_CACHE_INVALID = 0x27,
    LCB_SASLMECH_UNAVAILABLE = 0x28,
    LCB_TOO_MANY_REDIRECTS = 0x29,
    LCB_MAP_CHANGED = 0x2A,
    LCB_INCOMPLETE_PACKET = 0x2B,
    LCB_ECONNREFUSED = 0x2C,
    LCB_ESOCKSHUTDOWN = 0x2D,
    LCB_ECONNRESET = 0x2E,
    LCB_ECANTGETPORT = 0x2F,
    LCB_EFDLIMITREACHED = 0x30,
    LCB_ENETUNREACH = 0x31,
    LCB_ECTL_UNKNOWN = 0x32,
    LCB_ECTL_UNSUPPMODE = 0x33,
    LCB_ECTL_BADARG = 0x34,
    LCB_EMPTY_KEY = 0x35,
    LCB_SSL_ERROR = 0x36,
    LCB_SSL_CANTVERIFY = 0x37,
    LCB_SCHEDFAIL_INTERNAL = 0x38,
    LCB_CLIENT_FEATURE_UNAVAILABLE = 0x39,
    LCB_OPTIONS_CONFLICT = 0x3A,
    LCB_HTTP_ERROR = 0x3B,
    LCB_DURABILITY_NO_SYNCTOKEN = 0x3C,
    LCB_UNKNOWN_MEMCACHED_ERROR = 0x3D,
    LCB_MUTATION_LOST = 0x3E,
    LCB_MAX_ERROR = 0x1000,
}
impl Clone for lcb_error_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_error_t {
    fn default() -> Self { unsafe { zeroed() } }
}