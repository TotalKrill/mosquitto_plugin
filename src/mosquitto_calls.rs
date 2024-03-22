use crate::mosquitto_dev::{
    mosquitto_broker_publish, mosquitto_kick_client_by_clientid, mosquitto_kick_client_by_username,
    mosquitto_log_printf, mosquitto_property,
};
use crate::Error;
use crate::{Success, QOS};
use libc::c_void;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

/// Broadcast a message from the broker
/// If called in a username and password check the connecting client will not get the message
/// Use the publish_to_client combined with this if you want to send to all clients including the one that is connecting
pub fn publish_broadcast(
    topic: &str,
    payload: &[u8],
    qos: QOS,
    retain: bool,
) -> Result<Success, Error> {
    let cstr = &CString::new(topic).expect("no cstring for u");
    let bytes = cstr.as_bytes_with_nul();
    let topic = bytes.as_ptr();

    let nullptr: *const c_void = null();
    let properties: *mut mosquitto_property = std::ptr::null_mut();

    // let payload: *mut c_void = std::ptr::null_mut(); // payload bytes, non-null if payload length > 0, must be heap allocated
    let payload_len = payload.len();
    let payload: *const c_void = payload.as_ptr() as *const c_void; // payload bytes, non-null if payload length > 0, must be heap allocated

    unsafe {
        let c_payload: *mut c_void = libc::malloc(std::mem::size_of_val(&payload));
        payload.copy_to(c_payload, payload_len);
        /*
         * https://mosquitto.org/api2/files/mosquitto_broker-h.html#mosquitto_broker_publish
         * maybe want to switch to mosquitto_broker_publish to maintain ownership over
         * payload memory.
         * payload: payload bytes.  If payloadlen > 0 this must not be NULL.  Must be allocated on the heap.  Will be freed by mosquitto after use if the function returns success."
         * What happens if it is not successfull? Do i need to free the memory myself? This is a leak if if i dont' free memory  in all cases except 0 (Success) below?
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
            _default => Err(Error::Unknown),
        }
    }
}

/// To be called from implementations of the plugin when
/// a plugin wants to publish to a specific client.
pub fn publish_to_client(
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
    let payload: *const c_void = payload.as_ptr() as *const c_void;

    unsafe {
        let c_payload: *mut c_void = libc::malloc(std::mem::size_of_val(&payload));
        payload.copy_to(c_payload, payload_len);

        let res = mosquitto_broker_publish(
            client_id as *const c_char, // client id to send to, null = all clients
            topic as *const c_char,     // topic to publish on
            payload_len as i32,         // payload length in bytes, 0 for empty payload
            c_payload, // payload bytes, non-null if payload length > 0, must be heap allocated
            qos.to_i32(), // qos
            retain,    // retain
            std::ptr::null_mut(), //mqtt5 properties
        );
        match res {
            0 => Ok(Success),
            1 => Err(Error::NoMem),
            3 => Err(Error::Inval),
            _default => Err(Error::Unknown),
        }
    }
}

/// Forcefully disconnect all clients from the broker.
///
/// If `with_will` is true, then if the client has a Last Will and Testament
/// defined then this will be sent. If false, the LWT will not be sent.
pub fn kick_all_clients(with_will: bool) -> Result<Success, Error> {
    match unsafe { mosquitto_kick_client_by_clientid(null(), with_will) } {
        0 => Ok(Success),
        error => Err(Error::from(error)),
    }
}

/// Forcefully disconnect the client matching `client_id` from the broker.
///
/// If `with_will` is true, then if the client has a Last Will and Testament
/// defined then this will be sent. If false, the LWT will not be sent.
pub fn kick_client_by_clientid(client_id: &str, with_will: bool) -> Result<Success, Error> {
    let client_id = CString::new(client_id).map_err(|_| Error::Inval)?;
    match unsafe { mosquitto_kick_client_by_clientid(client_id.as_ptr(), with_will) } {
        0 => Ok(Success),
        error => Err(Error::from(error)),
    }
}

/// Forcefully disconnect the client connected with username `username` from the broker.
///
/// If `with_will` is true, then if the client has a Last Will and Testament
/// defined then this will be sent. If false, the LWT will not be sent.
pub fn kick_client_by_username(username: &str, with_will: bool) -> Result<Success, Error> {
    let username = CString::new(username).map_err(|_| Error::Inval)?;
    match unsafe { mosquitto_kick_client_by_username(username.as_ptr(), with_will) } {
        0 => Ok(Success),
        error => Err(Error::from(error)),
    }
}

/// Mosquitto log level.
#[repr(C)]
pub enum LogLevel {
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = 1 << 0,
    /// The "notice" level.
    ///
    /// Designates medium priority information.
    Notice = 1 << 1,
    /// The "warning" level.
    ///
    /// Designates hazardous situations.
    Warning = 1 << 2,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Err = 1 << 3,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = 1 << 4,
}

/// Send a log message on `level` to the mosquitto logging subsystem.
pub fn mosquitto_log(level: LogLevel, message: &str) {
    let message = CString::new(message).expect("invalid log message: contains a nul byte");
    unsafe {
        mosquitto_log_printf(
            level as i32,
            "%s\0".as_ptr() as *const c_char,
            message.as_ptr(),
        )
    }
}

/// Logs a message at the debug level into the mosquitto logging subsystem.
///
/// # Examples
///
/// ```no_run
/// use mosquitto_plugin::mosquitto_debug;
///
/// # fn main() {
/// let client_id = "unknown";
/// mosquitto_debug!("Authenticating client id {}", client_id);
/// # }
/// ```
#[macro_export]
macro_rules! mosquitto_debug {
    // mosquitto_debug!("a {} event", "log")
    ($($arg:tt)+) => ($crate::mosquitto_calls::mosquitto_log($crate::mosquitto_calls::LogLevel::Debug, &format!($($arg)+)))
}

/// Logs a message at the info level into the mosquitto logging subsystem.
///
/// # Examples
///
/// ```no_run
/// use mosquitto_plugin::mosquitto_info;
///
/// # fn main() {
/// let client_id = "unknown";
/// mosquitto_info!("Authentication of client id {} successful", client_id);
/// # }
/// ```
#[macro_export]
macro_rules! mosquitto_info {
    ($($arg:tt)+) => ($crate::mosquitto_calls::mosquitto_log($crate::mosquitto_calls::LogLevel::Info, &format!($($arg)+)))
}

/// Logs a message at the notice level into the mosquitto logging subsystem.
///
/// # Examples
///
/// ```no_run
/// use mosquitto_plugin::mosquitto_notice;
///
/// # fn main() {
/// let client_id = "unknown";
/// mosquitto_notice!("Authentication of client id {} is pending", client_id);
/// # }
/// ```
#[macro_export]
macro_rules! mosquitto_notice {
    ($($arg:tt)+) => ($crate::mosquitto_calls::mosquitto_log($crate::mosquitto_calls::LogLevel::Notice, &format!($($arg)+)))
}

/// Logs a message at the warn level into the mosquitto logging subsystem.
///
/// # Examples
///
/// ```no_run
/// use mosquitto_plugin::mosquitto_warn;
///
/// # fn main() {
/// let auth_method = "SIP";
/// let client_id = "unknown";
/// mosquitto_warn!("Client id {} tries unspported authentification method {}", client_id, auth_method);
/// # }
/// ```
#[macro_export]
macro_rules! mosquitto_warn {
    // mosquitto_warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::mosquitto_calls::mosquitto_log($crate::mosquitto_calls::LogLevel::Warning, &format!($($arg)+)))
}

/// Logs a message at the error level into the mosquitto logging subsystem.
///
/// # Examples
///
/// ```no_run
/// use mosquitto_plugin::mosquitto_error;
///
/// # fn main() {
/// let client_id: Option<&str> = None;
/// mosquitto_error!("Failed acl check for client id {:?}", client_id);
/// # }
/// ```
#[macro_export]
macro_rules! mosquitto_error {
    // mosquitto_error!("a {} event", "log")
    ($($arg:tt)+) => ($crate::mosquitto_calls::mosquitto_log($crate::mosquitto_calls::LogLevel::Err, &format!($($arg)+)))
}
