#![allow(clippy::not_unsafe_ptr_arg_deref)]

use mosquitto_plugin::*;
use std::collections::{HashMap, HashSet};

/// Random auth data that is expected from the client.
const HELLO_BROKER: &str = "hello broker";
/// Random auth data that is sent from the broker to the client.
const HELLO_CLIENT: &str = "hello client";

/// Example plugin for extended auth.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct AuthPlugin {
    /// Set of "authenticated" client ids.
    authenticated: HashSet<String>,
}

// Required trait implementation
impl MosquittoPlugin for AuthPlugin {
    fn init(_opts: HashMap<&str, &str>) -> Self {
        AuthPlugin::default()
    }

    fn on_disconnect(&mut self, client: &dyn MosquittoClientContext, _reason: i32) {
        println!(
            "Plugin on_disconnect, Client {:?} is disconnecting",
            client.get_id()
        );

        if let Some(ref client_id) = client.get_id() {
            self.authenticated.remove(client_id);
        }
    }

    fn acl_check(
        &mut self,
        client: &dyn MosquittoClientContext,
        _acl: AclCheckAccessLevel,
        _msg: MosquittoMessage,
    ) -> Result<Success, Error> {
        println!("Plugin on_acl_check, Client {:?}", client.get_id());
        let client_id = client.get_id().ok_or(Error::Inval)?;

        if self.authenticated.contains(&client_id) {
            Ok(Success)
        } else {
            Err(Error::Auth)
        }
    }

    /// Authentication start.
    fn on_auth_start(
        &mut self,
        client: &dyn MosquittoClientContext,
        method: Option<&str>,
        data: Option<&[u8]>,
    ) -> Result<Success, Error> {
        println!(
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
        println!(
            "Plugin on_auth_continue, Client {} with method {:?} and data {:?}",
            client_id, method, data
        );

        // If client replies with "hello broker" we're fine - otherwise greet again.
        if let Some(data) = data {
            if data == HELLO_BROKER.as_bytes() {
                println!(
                    "Plugin on_auth_continue, Client {} authenticated",
                    client_id
                );
                self.authenticated.insert(client_id.into());
                Ok(Success)
            } else {
                println!(
                    "Plugin on_auth_continue, Client {} failed to authenticate. Expected \"hello\"",
                    client_id
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
