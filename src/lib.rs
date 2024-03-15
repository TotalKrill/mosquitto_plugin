pub mod mosquitto_calls;
pub mod mosquitto_dev;

pub use mosquitto_dev::*;

use std::collections::HashMap;
use std::convert::From;
use std::ffi::CString;
use std::fmt;

pub mod dynlib;

pub use libc;
use std::net::IpAddr;
use std::str::FromStr;

pub type MosquittoOpt<'a> = HashMap<&'a str, &'a str>;

// parses the pointers given by mosquitto into a rust native structure
pub fn __from_ptr_and_size<'a>(opts: *mut mosquitto_opt, count: usize) -> MosquittoOpt<'a> {
    let mut map = HashMap::new();
    // Yep, raw pointer values
    let optsval = opts as usize;
    for i in 0..count {
        // manually increment the pointers according to the coun value
        let opt = unsafe {
            let opt = optsval + i * std::mem::size_of::<mosquitto_opt>();
            (opt as *mut mosquitto_opt)
                .as_ref()
                .expect("Failed to extract from ptr and size")
        };

        // get a reference, and then use this to parse the values into owned strings
        let key: &str = unsafe {
            let c_str = std::ffi::CStr::from_ptr(opt.key);
            c_str.to_str().expect("Failed to create string from CStr")
        };
        let value: &str = unsafe {
            let c_str = std::ffi::CStr::from_ptr(opt.value);
            c_str.to_str().expect("Failed to create string from CStr")
        };
        map.insert(key, value);
    }

    map
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessLevel {
    None = 0,
    Read = 1,
    Write = 2,
    Subscribe = 4,
    Unsubscribe = 8,
    Unknown,
}

impl From<i32> for AccessLevel {
    fn from(level: i32) -> AccessLevel {
        match level {
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
    Unsubscribe = 8,
}

impl std::fmt::Display for AclCheckAccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<AccessLevel> for Option<AclCheckAccessLevel> {
    fn from(level: AccessLevel) -> Option<AclCheckAccessLevel> {
        match level {
            AccessLevel::Read => Some(AclCheckAccessLevel::Read),
            AccessLevel::Write => Some(AclCheckAccessLevel::Write),
            AccessLevel::Subscribe => Some(AclCheckAccessLevel::Subscribe),
            AccessLevel::Unsubscribe => Some(AclCheckAccessLevel::Unsubscribe),
            _ => None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    AuthContinue(Vec<u8>),
    NoSubscriber,
    SubExists,
    ConnPending,
    NoMem,
    Protocol,
    Inval,
    NoConn,
    ConnRefused,
    NotFound,
    ConnLost,
    Tls,
    PayloadSize,
    NotSupported,
    Auth,
    AclDenied,
    Unknown,
    Errno,
    Eai,
    Proxy,
    PluginDefer,
    MalformedUtf8,
    Keepalive,
    Lookup,
    MalformedPacket,
    DuplicateProperty,
    TlsHandshake,
    QosNotSupported,
    OversizePacket,
    OCSP,
    Timeout,
    RetainNotSupported,
    TopicAliasInvalid,
    AdministrativeAction,
    AlreadyExists,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::AuthContinue(_) => -4,
            Error::NoSubscriber => -3,
            Error::SubExists => -2,
            Error::ConnPending => -1,
            Error::NoMem => 1,
            Error::Protocol => 2,
            Error::Inval => 3,
            Error::NoConn => 4,
            Error::ConnRefused => 5,
            Error::NotFound => 6,
            Error::ConnLost => 7,
            Error::Tls => 8,
            Error::PayloadSize => 9,
            Error::NotSupported => 10,
            Error::Auth => 11,
            Error::AclDenied => 12,
            Error::Unknown => 13,
            Error::Errno => 14,
            Error::Eai => 15,
            Error::Proxy => 16,
            Error::PluginDefer => 17,
            Error::MalformedUtf8 => 18,
            Error::Keepalive => 19,
            Error::Lookup => 20,
            Error::MalformedPacket => 21,
            Error::DuplicateProperty => 22,
            Error::TlsHandshake => 23,
            Error::QosNotSupported => 24,
            Error::OversizePacket => 25,
            Error::OCSP => 26,
            Error::Timeout => 27,
            Error::RetainNotSupported => 28,
            Error::TopicAliasInvalid => 29,
            Error::AdministrativeAction => 30,
            Error::AlreadyExists => 31,
        }
    }
}

impl From<i32> for Error {
    fn from(error: i32) -> Self {
        match error {
            -4 => Error::AuthContinue(Vec::with_capacity(0)),
            -3 => Error::NoSubscriber,
            -2 => Error::SubExists,
            -1 => Error::ConnPending,
            1 => Error::NoMem,
            2 => Error::Protocol,
            3 => Error::Inval,
            4 => Error::NoConn,
            5 => Error::ConnRefused,
            6 => Error::NotFound,
            7 => Error::ConnLost,
            8 => Error::Tls,
            9 => Error::PayloadSize,
            10 => Error::NotSupported,
            11 => Error::Auth,
            12 => Error::AclDenied,
            13 => Error::Unknown,
            14 => Error::Errno,
            15 => Error::Eai,
            16 => Error::Proxy,
            17 => Error::PluginDefer,
            18 => Error::MalformedUtf8,
            19 => Error::Keepalive,
            20 => Error::Lookup,
            21 => Error::MalformedPacket,
            22 => Error::DuplicateProperty,
            23 => Error::TlsHandshake,
            24 => Error::QosNotSupported,
            25 => Error::OversizePacket,
            26 => Error::OCSP,
            27 => Error::Timeout,
            28 => Error::RetainNotSupported,
            29 => Error::TopicAliasInvalid,
            30 => Error::AdministrativeAction,
            31 => Error::AlreadyExists,
            _ => Error::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Success;

impl From<Success> for i32 {
    fn from(_: Success) -> i32 {
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
    pub fn to_i32(&self) -> i32 {
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

    /// NOTE: stored sessions might be disconnected upon a restart, and then the client being
    /// disconnected will have no IP address, the address will then be of type None
    fn get_address(&self) -> Option<std::net::IpAddr>;
    /// Binding to mosquitto_client_clean_session
    fn is_clean_session(&self) -> bool;
    /// Binding to mosquitto_client_id
    fn get_id(&self) -> Option<String>;
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
    fn get_username(&self) -> Option<String>;
    /// Binding to mosquitto_set_username
    /// Error is either NoMem or Inval
    fn set_username(&self, username: String) -> Result<Success, Error>;
}

pub struct MosquittoClient {
    pub client: *mut mosquitto,
}

impl MosquittoClientContext for MosquittoClient {
    fn get_address(&self) -> Option<IpAddr> {
        unsafe {
            debug_assert!(!self.client.is_null(), "get_address: self client is null");
            let address = mosquitto_client_address(self.client);
            if address.is_null() {
                None
            } else {
                let c_str = std::ffi::CStr::from_ptr(address);
                let str = c_str.to_str().expect("Couldn't convert CStr to &str"); // TODO should we avoid expect here and instead return Option<String>?
                Some(IpAddr::from_str(str).expect("Couldn't parse ip"))
            }
        }
    }

    fn is_clean_session(&self) -> bool {
        debug_assert!(
            !self.client.is_null(),
            "is_clean_session: self client is null"
        );
        unsafe { mosquitto_client_clean_session(self.client) }
    }

    fn get_id(&self) -> Option<String> {
        debug_assert!(!self.client.is_null(), "get_id: self client is null");
        unsafe {
            let client_id = mosquitto_client_id(self.client);

            if client_id.is_null() {
                None
            } else {
                let c_str = std::ffi::CStr::from_ptr(client_id);
                let r_string = c_str
                    .to_str()
                    .expect("Couldn't convert CStr to &str")
                    .to_string(); // TODO should we avoid expect here and instead return Option<String>?
                Some(r_string)
            }
        }
    }

    fn get_keepalive(&self) -> i32 {
        debug_assert!(!self.client.is_null(), "get_keepalive: self client is null");
        unsafe { mosquitto_client_keepalive(self.client) }
    }

    fn get_certificate(&self) -> Option<&[u8]> {
        unimplemented!()
    }

    fn get_protocol(&self) -> MosquittoClientProtocol {
        debug_assert!(!self.client.is_null(), "get_protocol: self client is null");
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

                panic!(
                    "mosquitto_client_protocol returned invalid protocol {}",
                    protocol
                );
            }
        }
    }

    fn get_protocol_version(&self) -> MosquittoClientProtocolVersion {
        debug_assert!(
            !self.client.is_null(),
            "get_protocol_version: self client is null"
        );
        unsafe {
            let protocol_version = mosquitto_client_protocol_version(self.client);
            match protocol_version {
                3 => MosquittoClientProtocolVersion::V3,
                4 => MosquittoClientProtocolVersion::V4,
                5 => MosquittoClientProtocolVersion::V5,
                _ => panic!(
                    "invalid mosquitto client protocol version returned from mosquitto_client_protocol_version. {}",
                    protocol_version
                ),
            }
        }
    }

    fn get_sub_count(&self) -> i32 {
        debug_assert!(!self.client.is_null(), "get_sub_count: self client is null");
        unsafe { mosquitto_client_sub_count(self.client) as i32 }
    }

    fn get_username(&self) -> Option<String> {
        debug_assert!(!self.client.is_null(), "get_username: self client is null");
        unsafe {
            let username = mosquitto_client_username(self.client);
            if username.is_null() {
                None
            } else {
                let c_str = std::ffi::CStr::from_ptr(username);
                c_str.to_str().ok().map(From::from)
            }
        }
    }

    fn set_username(&self, username: String) -> Result<Success, Error> {
        debug_assert!(!self.client.is_null(), "set_username: self client is null");
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

    /// Authentication start. Default implementation always returns success. Return Err(Error::AuthContinue(_))
    /// to send auth data to the client.
    fn on_auth_start(
        &mut self,
        _client: &dyn MosquittoClientContext,
        _method: Option<&str>,
        _data: Option<&[u8]>,
    ) -> Result<Success, Error> {
        Ok(Success)
    }

    /// Authentication continue. Default implementation always returns success. Return `Err(Error::AuthContinue(_))`
    /// to send auth data to the client or `Ok(Success)`.
    fn on_auth_continue(
        &mut self,
        _client: &dyn MosquittoClientContext,
        _method: Option<&str>,
        _data: Option<&[u8]>,
    ) -> Result<Success, Error> {
        Ok(Success)
    }

    /// Tested unsuccessfully. Haven't gotten this to work yet.
    /// Suspect it has something to do with how the mosquitto_callback_register is called with the event_data parameter
    #[allow(unused)]
    fn on_control(&mut self, client: &dyn MosquittoClientContext, message: MosquittoMessage) {}

    /// Called when a message is sent on the broker.
    /// The message has to pass the ACL check otherwise this callback will not be called.
    #[allow(unused)]
    fn on_message(&mut self, client: &dyn MosquittoClientContext, message: MosquittoMessage) {}

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
        debug_assert_eq!(2 + 2, 4);
    }
}
