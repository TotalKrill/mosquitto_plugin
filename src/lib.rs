pub mod mosquitto_dev;

pub use mosquitto_dev::{
    mosquitto, mosquitto_acl_msg, mosquitto_broker_publish, mosquitto_opt, mosquitto_property,
    mqtt5__property,
};

use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
pub mod dynlib;
pub use dynlib::*;
pub use libc;
use libc::c_void;

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

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessLevel {
    None = 0,
    Read = 1,
    Write = 2,
    Subscribe = 4,
    Unsubscribe = 8,
    Unknown,
}

impl Into<AccessLevel> for i32 {
    fn into(self) -> AccessLevel {
        match self {
            0 => AccessLevel::None,
            1 => AccessLevel::Read,
            2 => AccessLevel::Write,
            4 => AccessLevel::Subscribe,
            8 => AccessLevel::Unsubscribe,
            _ => AccessLevel::Unknown,
        }
    }
}

impl std::fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum AclCheckAccessLevel {
    Read = 1,
    Write = 2,
    Subscribe = 4,
}

impl std::fmt::Display for AclCheckAccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<Option<AclCheckAccessLevel>> for AccessLevel {
    fn into(self) -> Option<AclCheckAccessLevel> {
        match self {
            AccessLevel::Read => Some(AclCheckAccessLevel::Read),
            AccessLevel::Write => Some(AclCheckAccessLevel::Write),
            AccessLevel::Subscribe => Some(AclCheckAccessLevel::Subscribe),
            _ => None,
        }
    }
}

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

pub enum QOS {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}
impl QOS {
    fn to_i32(&self) -> i32 {
        match self {
            QOS::AtMostOnce => 0,
            QOS::AtLeastOnce => 1,
            QOS::ExactlyOnce => 2,
        }
    }
}

pub trait MosquittoPlugin {
    /// This will be run once on every startup, or load, and will allocate the structure, to be
    /// reconstructed in other calls to the plugin.
    ///
    /// This requires unsafe usage due to nature of C calls
    fn init(opts: MosquittoOpt) -> Self;

    /// Access level checks, default implementation always returns success
    #[allow(unused)]
    fn acl_check(
        &mut self,
        client_id: &str,
        acl: AclCheckAccessLevel,
        msg: MosquittoAclMessage,
    ) -> Result<Success, Error> {
        Ok(Success)
    }
    #[allow(unused)]
    /// Username and password checks, default implementation always returns success
    fn username_password(
        &mut self,
        client_id: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Success, Error> {
        Ok(Success)
    }
    #[allow(unused)]
    /// Broadcast a message from the broker
    /// If called in a username and password check the connecting client will not get the message
    /// Use the broker_publish_to_client combined with this if you want to send to all clients including the one that is connecting
    fn broker_broadcast_publish(
        &mut self,
        topic: &str,
        payload: &[u8],
        qos: QOS,
        retain: bool,
    ) -> Result<Success, Error> {
        let cstr = &CString::new(topic).expect("no cstring for u");
        let bytes = cstr.as_bytes_with_nul();
        let topic = bytes.as_ptr();

        let mut nullptr: *const c_void = std::ptr::null();
        let properties: *mut mosquitto_property = std::ptr::null_mut();

        // let payload: *mut c_void = std::ptr::null_mut(); // payload bytes, non-null if payload length > 0, must be heap allocated
        let payload_len = payload.len();
        let payload: *const c_void = Box::new(payload).as_ptr() as *const c_void; // payload bytes, non-null if payload length > 0, must be heap allocated

        unsafe {
            let c_payload: *mut c_void =
                libc::malloc(std::mem::size_of::<u8>() * payload_len) as *mut c_void;
            payload.copy_to(c_payload, payload_len);
            /**
             * https://mosquitto.org/api2/files/mosquitto_broker-h.html#mosquitto_broker_publish
             * maybe want to switch to mosquitto_broker_publish to maintain ownership over
             * payload memory.
             * "payload	payload bytes.  If payloadlen > 0 this must not be NULL.  Must be allocated on the heap.  Will be freed by mosquitto after use if the function returns success."
             * What happens if it is not successfull? Do i need to free the memory myself? This is a leak if if i front free memory  in all cases except 0 (Success) below?
             */
            let res = mosquitto_broker_publish(
                nullptr as *const i8, // client id to send to, null = all clients
                topic as *const i8,
                payload_len as i32, // payload length in bytes, 0 for empty payload
                c_payload, // payload bytes, non-null if payload length > 0, must be heap allocated
                qos.to_i32(), // qos
                retain,    // retain
                properties, //mqtt5 properties
            );
            match res {
                0 => Ok(Success),
                1 => Err(Error::NoMem),
                3 => Err(Error::Inval),
                default => Err(Error::Unknown),
            }
        }
    }
    #[allow(unused)]
    /// To be called from implementations the plugin when
    /// a plugin wants to publish to a specific client.
    fn broker_publish_to_client(
        &mut self,
        client_id: &str,
        topic: &str,
        payload: &[u8],
        qos: QOS,
        retain: bool,
    ) -> Result<Success, Error> {
        let cstr = &CString::new(client_id).expect("no cstring for u");
        let bytes = cstr.as_bytes_with_nul();
        let client_id = bytes.as_ptr();

        let cstr = &CString::new(topic).expect("no cstring for u");
        let bytes = cstr.as_bytes_with_nul();
        let topic = bytes.as_ptr();

        let payload_len = payload.len();
        let payload: *const c_void = Box::new(payload).as_ptr() as *const c_void;

        unsafe {
            let c_payload: *mut c_void =
                libc::malloc(std::mem::size_of::<u8>() * payload_len) as *mut c_void;
            payload.copy_to(c_payload, payload_len);

            let res = mosquitto_broker_publish(
                client_id as *const i8, // client id to send to, null = all clients
                topic as *const i8,     // topic to publish on
                payload_len as i32,     // payload length in bytes, 0 for empty payload
                c_payload, // payload bytes, non-null if payload length > 0, must be heap allocated
                qos.to_i32(), // qos
                retain,    // retain
                std::ptr::null_mut(), //mqtt5 properties
            );
            match res {
                0 => Ok(Success),
                1 => Err(Error::NoMem),
                3 => Err(Error::Inval),
                default => Err(Error::Unknown),
            }
        }
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
