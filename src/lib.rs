pub mod mosquitto_dev;

pub use mosquitto_dev::*;

use std::collections::HashMap;
use std::convert::From;
use std::ffi::{CString, CStr};
use std::fmt;
use std::os::raw::c_char;

pub mod dynlib;

pub use dynlib::*;
pub use libc;
use libc::c_void;
use std::net::IpAddr;
use std::str::FromStr;

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
pub struct MosquittoMessage<'a> {
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

pub enum MosquittoClientProtocol {
    Mqtt,
    MqttSn,
    Websockets,
}

pub enum MosquittoClientProtocolVersion {
    V3,
    V4,
    V5,
}

pub trait MosquittoClientContext {
    /// Binding to mosquitto_client_address
    fn get_address(&self) -> std::net::IpAddr;
    /// Binding to mosquitto_client_clean_session
    fn is_clean_session(&self) -> bool;
    /// Binding to mosquitto_client_id
    fn get_id(&self) -> String;
    /// Binding to mosquitto_client_keepalive
    fn get_keepalive(&self) -> i32;
    /// Binding to mosquitto_client_certificate
    fn get_certificate(&self) -> Option<&[u8]>;
    // TODO replace with a reasonable return type from another lib. openssl or x509_parser maybe?
    /// Binding to mosquitto_client_protocol
    fn get_protocol(&self) -> MosquittoClientProtocol;
    /// Binding to mosquitto_client_protocol_version
    fn get_protocol_version(&self) -> MosquittoClientProtocolVersion;
    /// Binding to mosquitto_client_sub_count
    fn get_sub_count(&self) -> i32;
    /// Binding to mosquitto_client_username
    fn get_username(&self) -> String;
    /// Binding to mosquitto_set_username
    /// Error is either NoMem or Inval
    fn set_username(&self, username: String) -> Result<Success, Error>;
}

pub struct MosquittoClient {
    pub client: *mut mosquitto
}

impl MosquittoClientContext for MosquittoClient {
    fn get_address(&self) -> IpAddr {
        unsafe {
            let address = mosquitto_client_address(self.client);
            let c_str = std::ffi::CStr::from_ptr(address);
            let str = c_str.to_str().expect("Couldn't convert CStr to &str"); // TODO should we avoid expect here and instead return Option<String>?
            IpAddr::from_str(str).expect("Couldn't parse ip")
        }
    }

    fn is_clean_session(&self) -> bool {
        unsafe {
            mosquitto_client_clean_session(self.client)
        }
    }

    fn get_id(&self) -> String {
        unsafe {
            let client_id = mosquitto_client_id(self.client);
            let c_str = std::ffi::CStr::from_ptr(client_id);
            c_str.to_str().expect("Couldn't convert CStr to &str").to_string() // TODO should we avoid expect here and instead return Option<String>?
        }
    }

    fn get_keepalive(&self) -> i32 {
        unsafe {
            mosquitto_client_keepalive(self.client)
        }
    }

    fn get_certificate(&self) -> Option<&[u8]> {
        unimplemented!()
    }

    fn get_protocol(&self) -> MosquittoClientProtocol {
        unsafe {
            let protocol = mosquitto_client_protocol(self.client) as u32;
            if protocol == mosquitto_protocol_mp_mqtt {
                MosquittoClientProtocol::Mqtt
            } else if protocol == mosquitto_protocol_mp_mqttsn {
                MosquittoClientProtocol::MqttSn
            } else if protocol == mosquitto_protocol_mp_websockets {
                MosquittoClientProtocol::Websockets
            } else {
                // TODO either we panic here. Or we need to return a result/option
                // The benefit of returning the result/option would be to let library-user-space
                // gracefully shutdown. Which would be preferable.

                panic!("mosquitto_client_protocol returned invalid protocol {}", protocol);
            }
        }
    }

    fn get_protocol_version(&self) -> MosquittoClientProtocolVersion {
        unsafe {
            let protocol_version = mosquitto_client_protocol_version(self.client);
            match protocol_version {
                3 => MosquittoClientProtocolVersion::V3,
                4 => MosquittoClientProtocolVersion::V4,
                5 => MosquittoClientProtocolVersion::V5,
                _ => panic!("invalid mosquitto client protocol version returned from mosquitto_client_protocol_version. {}", protocol_version)
            }
        }
    }

    fn get_sub_count(&self) -> i32 {
        unsafe {
            mosquitto_client_sub_count(self.client) as i32
        }
    }

    fn get_username(&self) -> String {
        unsafe {
            let username = mosquitto_client_username(self.client);
            let c_str = std::ffi::CStr::from_ptr(username);
            c_str.to_str().expect("Couldn't convert CStr to &str").to_string() // TODO should we avoid expect here and instead return Option<String>?
        }
    }

    fn set_username(&self, username: String) -> Result<Success, Error> {
        unsafe {
            let c_string = &CString::new(username).expect("no cstring for u");
            let res = mosquitto_set_username(self.client, c_string.as_c_str().as_ptr());
            match res {
                0 => Ok(Success),
                1 => Err(Error::NoMem),
                3 => Err(Error::Inval),
                _ => Err(Error::Unknown), // Any other number returned from set_username is undefined behaviour
            }
        }
    }
}

#[derive(Debug)]
pub enum MosquittoPluginEvent {
    MosqEvtReload = 1,
    MosqEvtAclCheck = 2,
    MosqEvtBasicAuth = 3,
    MosqEvtExtAuthStart = 4,
    MosqEvtExtAuthContinue = 5,
    MosqEvtControl = 6,
    MosqEvtMessage = 7,
    MosqEvtPskKey = 8,
    MosqEvtTick = 9,
    MosqEvtDisconnect = 10,
    Unknown = -1,
}

impl From<MosquittoPluginEvent> for i32 {
    fn from(it: MosquittoPluginEvent) -> i32 {
        match it {
            MosquittoPluginEvent::MosqEvtReload => 1,
            MosquittoPluginEvent::MosqEvtAclCheck => 2,
            MosquittoPluginEvent::MosqEvtBasicAuth => 3,
            MosquittoPluginEvent::MosqEvtExtAuthStart => 4,
            MosquittoPluginEvent::MosqEvtExtAuthContinue => 5,
            MosquittoPluginEvent::MosqEvtControl => 6,
            MosquittoPluginEvent::MosqEvtMessage => 7,
            MosquittoPluginEvent::MosqEvtPskKey => 8,
            MosquittoPluginEvent::MosqEvtTick => 9,
            MosquittoPluginEvent::MosqEvtDisconnect => 10,
            MosquittoPluginEvent::Unknown => -1,
        }
    }
}

pub trait MosquittoPlugin {
    /// This will be run once on every startup, or load, and will allocate the structure, to be
    /// reconstructed in other calls to the plugin.
    ///
    /// This requires unsafe usage due to nature of C calls
    fn init(opts: MosquittoOpt) -> Self;

    /// Called when SIGHUP is sent to the broker PID
    #[allow(unused)]
    fn on_reload(&mut self, opts: MosquittoOpt) {}

    /// Access level checks, default implementation always returns success
    /// If all acl checks from all plugins returns defer the action should be allowed.
    /// However that doesn't happen right now, if this returns Err(PluginDefer) for a write the message is not let through.
    #[allow(unused)]
    fn acl_check(
        &mut self,
        client: &dyn MosquittoClientContext,
        acl: AclCheckAccessLevel,
        msg: MosquittoMessage,
    ) -> Result<Success, Error> {
        Ok(Success)
    }
    #[allow(unused)]
    /// Username and password checks, default implementation always returns success
    fn username_password(
        &mut self,
        client: &dyn MosquittoClientContext,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Success, Error> {
        Ok(Success)
    }

    /// Tested unsuccessfully. Haven't gotten this to work yet.
    /// Suspect it has something to do with how the mosquitto_callback_register is called with the event_data parameter
    #[allow(unused)]
    fn on_control(
        &mut self,
        client: &dyn MosquittoClientContext,
        message: MosquittoMessage,
    ) {}

    /// Called when a message is sent on the broker.
    /// The message has to pass the ACL check otherwise this callback will not be called.
    #[allow(unused)]
    fn on_message(
        &mut self,
        client: &dyn MosquittoClientContext,
        message: MosquittoMessage,
    ) {}

    /// Untested
    #[allow(unused)]
    fn on_psk(
        &mut self,
        client: &dyn MosquittoClientContext,
        hint: &str,
        identity: &str,
        key: &str,
        max_key_len: i32,
    ) -> i32 {
        0
    }

    /// Called every 100 ms
    /// All now_ns, next_ns, now_s, next_s parameters are always zero right now.
    /// I'm not sure if it's a bug on this library's part or of mosquitto.
    /// If you want to keep time you'll have to measure it yourself right now.
    #[allow(unused)]
    fn on_tick(&mut self, now_ns: i64, next_ns: i64, now_s: i32, next_s: i32) {}

    #[allow(unused)]
    fn on_disconnect(&mut self, client: &dyn MosquittoClientContext, reason: i32) {}

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
                nullptr as *const c_char, // client id to send to, null = all clients
                topic as *const c_char,
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
    /// To be called from implementations of the plugin when
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
                client_id as *const c_char, // client id to send to, null = all clients
                topic as *const c_char,     // topic to publish on
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
