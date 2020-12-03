pub mod mosquitto_dev;

pub use mosquitto_dev::{mosquitto, mosquitto_acl_msg, mosquitto_opt};

use std::collections::HashMap;
use std::ffi::CStr;
pub mod dynlib;
pub use dynlib::*;
pub use libc;

pub fn __own_string(ch: *mut std::os::raw::c_char) -> String {
    if !ch.is_null() {
        unsafe { CStr::from_ptr(ch).to_string_lossy().into_owned() }
    } else {
        "".to_string()
    }
}

pub type MosquittoOpt<'a> = HashMap<&'a str, &'a str>;

// parses the pointers given by mosquitto into a rust native structure
pub fn __from_ptr_and_size<'a>(opts: *mut mosquitto_opt, count: usize) -> MosquittoOpt<'a> {
    let mut map = HashMap::new();
    // Yep, raw pointer values
    let optsval = opts as usize;
    for i in 0..count {
        // manually increment the pointers according to the coun value
        let opt = unsafe {
            let opt = optsval + i as usize * std::mem::size_of::<mosquitto_opt>();
            (opt as *mut mosquitto_opt).as_ref().unwrap()
        };

        // get a reference, and then use this to parse the values into owned strings
        //let key = __own_string(opt.key);
        let key: &str = unsafe {
            let c_str = std::ffi::CStr::from_ptr(opt.key);
            c_str.to_str().unwrap()
        };
        let value: &str = unsafe {
            let c_str = std::ffi::CStr::from_ptr(opt.value);
            c_str.to_str().unwrap()
        };
        map.insert(key, value);
    }

    map
}

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub enum AccessLevel {
//     Subscribe = 0,
//     Read = 1,
//     Write = 2,
//     None = 3,
// }
// impl Into<AccessLevel> for i32 {
//     fn into(self) -> AccessLevel {
//         match self {
//             0 => AccessLevel::Subscribe,
//             1 => AccessLevel::Read,
//             2 => AccessLevel::Write,
//             _ => AccessLevel::None,
//         }
//     }
// }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Error {
    AuthContinue = -4,
    NoSubscriber = -3,
    SubExists = -2,
    ConnPending = -1,
    NoMem = 1,
    Protocol = 2,
    Inval = 3,
    NoConn = 4,
    ConnRefused = 5,
    NotFound = 6,
    ConnLost = 7,
    Tls = 8,
    PayloadSize = 9,
    NotSupported = 10,
    Auth = 11,
    AclDenied = 12,
    Unknown = 13,
    Errno = 14,
    Eai = 15,
    Proxy = 16,
    PluginDefer = 17,
    MalformedUtf8 = 18,
    Keepalive = 19,
    Lookup = 20,
    MalformedPacket = 21,
    DuplicateProperty = 22,
    TlsHandshake = 23,
    QosNotSupported = 24,
    OversizePacket = 25,
    OCSP = 26,
}

impl Into<i32> for Error {
    fn into(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Success;
impl Into<i32> for Success {
    fn into(self) -> i32 {
        0
    }
}

// #[repr(C)]
// #[derive(Debug)]
// pub enum QoS {
//     AtMostOnce = 0,
//     AtLeastOnce = 1,
//     ExactlyOnce = 2,
// }

// impl QoS {
//     pub fn from_num(n: i32) -> Self {
//         match n {
//             2 => QoS::ExactlyOnce,
//             1 => QoS::AtLeastOnce,
//             _ => QoS::AtMostOnce,
//         }
//     }
// }

#[derive(Debug)]
pub struct MosquittoAclMessage<'a> {
    pub topic: &'a str,
    pub payload: &'a [u8],
    pub qos: i32,
    pub retain: bool,
}

pub trait MosquittoPlugin {
    /// This will be run once on every startup, or load, and will allocate the structure, to be
    /// reconstructed in other calls to the plugin.
    ///
    /// This requires unsafe usage due to nature of C calls
    fn init(opts: MosquittoOpt) -> Self;

    /// Access level checks, default implementation always returns success
    #[allow(unused)]
    fn acl_check(&mut self, acl: i32, msg: MosquittoAclMessage) -> Result<Success, Error> {
        Ok(Success)
    }
    #[allow(unused)]
    /// Username and password checks, default implementation always returns success
    fn username_password(&mut self, username: &str, password: &str) -> Result<Success, Error> {
        Ok(Success)
    }
}

// #[derive(Debug)]
// pub struct Test {
//     i: i32,
//     s: String,
// }

// impl MosquittoPlugin for Test {
//     fn init(opts: MosquittoOpt) -> Self {
//         Test {
//             i: 32,
//             s: "I am a struct string, wants to be recreated".into(),
//         }
//     }

//     fn username_password(&mut self, u: String, p: String) -> Result<Success, Error> {
//         self.s = "Well, i changed, whatyagonnadoaboutit".into();

//         if u == "hej" && p == "nope" {
//             Ok(Success)
//         } else {
//             Err(Error::Auth)
//         }
//     }
// }
// create_dynamic_library!(Test);

// #[no_mangle]
// pub extern "C" fn mosquitto_auth_psk_key_get(
//     user_data: *mut Void,
//     client: *mut mosquitto,
//     hint: *const Char,
//     identity: *const Char,
//     key: *mut Char,
//     max_key_len: Int,
// ) -> Int {
//     0
// }
// #[no_mangle]
// pub extern "C" fn mosquitto_auth_start(
//     user_data: *mut Void,
//     client: *mut mosquitto,
//     method: *const Char,
//     reauth: bool,
//     data_in: *const Void,
//     data_in_len: u16,
//     data_out: *mut *mut Void,
//     data_out_len: *mut u16,
// ) -> Int {
//     0
// }
// #[no_mangle]
// pub extern "C" fn mosquitto_auth_continue(
//     user_data: *mut Void,
//     client: *mut mosquitto,
//     method: *const Char,
//     data_in: *const Void,
//     data_in_len: u16,
//     data_out: *mut *mut Void,
//     data_out_len: *mut u16,
// ) -> Int {
//     0
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
