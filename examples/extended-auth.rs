#![allow(clippy::not_unsafe_ptr_arg_deref)]

use mosquitto_plugin::*;
use std::collections::HashMap;

/// Random auth data that is expected from the client.
const HELLO_BROKER: &str = "hello broker";
/// Random auth data that is sent from the broker to the client.
const HELLO_CLIENT: &str = "hello client";

/// Example plugin for extended auth.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct AuthPlugin;

// Required trait implementation
impl MosquittoPlugin for AuthPlugin {
    fn init(_opts: HashMap<&str, &str>) -> Self {
        AuthPlugin
    }

    fn on_disconnect(&mut self, client: &dyn MosquittoClientContext, _reason: i32) {
        mosquitto_info!(
            "Plugin on_disconnect, Client {:?} is disconnecting",
            client.get_id()
        );
    }

    /// Authentication start.
    fn on_auth_start(
        &mut self,
        client: &dyn MosquittoClientContext,
        method: Option<&str>,
        data: Option<&[u8]>,
    ) -> Result<Success, Error> {
        mosquitto_info!(
            "Plugin on_auth_start, Client {:?} with method {:?} and data {:?}",
            client.get_id(),
            method,
            data
        );
        // Sending random auth data "hello"
        Err(Error::AuthContinue(HELLO_CLIENT.as_bytes().to_vec()))
    }

    /// Authentication continue.
    fn on_auth_continue(
        &mut self,
        client: &dyn MosquittoClientContext,
        method: Option<&str>,
        data: Option<&[u8]>,
    ) -> Result<Success, Error> {
        let client_id = client.get_id().ok_or(Error::Inval)?;
        mosquitto_info!(
            "Plugin on_auth_continue, Client {} with method {:?} and data {:?}",
            client_id,
            method,
            data
        );

        // If client replies with "hello broker" we're fine - otherwise greet again.
        if let Some(data) = data {
            if data == HELLO_BROKER.as_bytes() {
                mosquitto_info!(
                    "Plugin on_auth_continue, Client {} authenticated",
                    client_id
                );
                Ok(Success)
            } else {
                mosquitto_warn!(
                    "Plugin on_auth_continue, Client {} failed to authenticate. Expected \"{}\"",
                    client_id,
                    HELLO_BROKER,
                );
                Err(Error::AuthContinue(HELLO_CLIENT.as_bytes().to_vec()))
            }
        } else {
            Err(Error::AuthContinue(HELLO_CLIENT.as_bytes().to_vec()))
        }
    }
}

// This generates the dynamic c bindings functions that are exported and usable by mosquitto
create_dynamic_library!(AuthPlugin);
