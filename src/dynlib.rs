// This generates the dynamic c bindings functions that are exported and usable by mosquitto, as
// well as allocating memory for the structure and handles recreating this from raw memory, to
// allow the generated plugin to use member functions, and thus have mutable state
#[macro_export]
macro_rules! create_dynamic_library {
    ($t:ty) => {
        use mosquitto_dev::*;
        use std::os::raw::c_int;
        use std::os::raw::c_void;

        fn __assert_sync()
        where
            $t: MosquittoPlugin,
        {
        }

        /// Structure internal to the plugin binder.
        /// identifier is the plugin identifier recievied in mosquitto_plugin_init
        /// external_user_data is the struct defined by the library user.
        struct InternalUserData {
            identifier: *mut c_void,
            external_user_data: $t,
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_plugin_version() -> isize {
            mosquitto_dev::MOSQ_PLUGIN_VERSION as isize
        }

        // Trampoline functions that are used as callback for the mosquitto_callback_register
        // These function satisfy the types of the C bindings and then call their corresponding safer rust calls.

        #[no_mangle]
        extern "C" fn on_reload_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_reload =
                unsafe { &mut *(event_data as *mut mosquitto_evt_reload) };
            let opts = __from_ptr_and_size(event_data.options, event_data.option_count as _);
            user_data.external_user_data.on_reload(opts);
            0
        }

        #[no_mangle]
        extern "C" fn on_acl_check_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_acl_check =
                unsafe { &mut *(event_data as *mut mosquitto_evt_acl_check) };
            let access_level: AccessLevel = event_data.access.into();
            let access_level = if let Some(level) = access_level.into() {
                level
            } else {
                println!("Unexpected access level for acl check. {:?}", access_level);
                return Error::Unknown.into();
            };

            let topic: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(event_data.topic);
                c_str
                    .to_str()
                    .expect("Acl trampoline, failed to create &str from CStr pointer")
            };

            let payload: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    event_data.payload as *const u8,
                    event_data.payloadlen as usize,
                )
            };

            let msg = MosquittoMessage {
                topic,
                payload,
                qos: event_data.qos.into(),
                retain: event_data.retain,
            };
            match user_data.external_user_data.acl_check(
                &MosquittoClient {
                    client: event_data.client,
                },
                access_level,
                msg,
            ) {
                Ok(s) => s.into(),
                Err(e) => e.into(),
            }
        }

        #[no_mangle]
        extern "C" fn on_basic_auth_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_basic_auth =
                unsafe { &mut *(event_data as *mut mosquitto_evt_basic_auth) };
            let username: Option<&str> = unsafe {
                if event_data.username.is_null() {
                    None
                } else {
                    let c_str = std::ffi::CStr::from_ptr(event_data.username);
                    Some(c_str.to_str().expect(
                        "basic auth trampoline failed to create username &str from CStr pointer",
                    ))
                }
            };
            let password: Option<&str> = unsafe {
                if event_data.password.is_null() {
                    None
                } else {
                    let c_str = std::ffi::CStr::from_ptr(event_data.password);
                    Some(c_str.to_str().expect(
                        "basic auth trampoline failed to create password &str from CStr pointer",
                    ))
                }
            };

            match user_data.external_user_data.username_password(
                &MosquittoClient {
                    client: event_data.client,
                },
                username,
                password,
            ) {
                Ok(r) => r.into(),
                Err(e) => e.into(),
            }
        }

        #[no_mangle]
        extern "C" fn on_auth_start_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_extended_auth =
                unsafe { &mut *(event_data as *mut mosquitto_evt_extended_auth) };
            unimplemented!();
        }

        #[no_mangle]
        extern "C" fn on_auth_continue_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_extended_auth =
                unsafe { &mut *(event_data as *mut mosquitto_evt_extended_auth) };
            unimplemented!();
        }

        #[no_mangle]
        extern "C" fn on_control_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_control =
                unsafe { &mut *(event_data as *mut mosquitto_evt_control) };
            let topic: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(event_data.topic);
                c_str
                    .to_str()
                    .expect("control trampoline failed to create topic &str from CStr pointer")
            };

            let payload: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    event_data.payload as *const u8,
                    event_data.payloadlen as usize,
                )
            };

            let msg = MosquittoMessage {
                topic,
                payload,
                qos: event_data.qos.into(),
                retain: event_data.retain,
            };

            user_data.external_user_data.on_control(
                &MosquittoClient {
                    client: event_data.client,
                },
                msg,
            );
            0
        }

        #[no_mangle]
        extern "C" fn on_message_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_message =
                unsafe { &mut *(event_data as *mut mosquitto_evt_message) };
            let topic: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(event_data.topic);
                c_str
                    .to_str()
                    .expect("message trampoline failed to create topic &str from CStr pointer")
            };

            let payload: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    event_data.payload as *const u8,
                    event_data.payloadlen as usize,
                )
            };

            let msg = MosquittoMessage {
                topic,
                payload,
                qos: event_data.qos.into(),
                retain: event_data.retain,
            };

            user_data.external_user_data.on_message(
                &MosquittoClient {
                    client: event_data.client,
                },
                msg,
            );
            0
        }

        #[no_mangle]
        extern "C" fn on_psk_key_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_psk_key =
                unsafe { &mut *(event_data as *mut mosquitto_evt_psk_key) };

            let hint: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(event_data.hint);
                c_str
                    .to_str()
                    .expect("psk key trampoline failed to create hint &str from CStr pointer")
            };

            let identity: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(event_data.identity);
                c_str
                    .to_str()
                    .expect("psk key trampoline failed to create identity &str from CStr pointer")
            };

            let key: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(event_data.key);
                c_str
                    .to_str()
                    .expect("psk key trampoline failed to create key &str from CStr pointer")
            };

            user_data.external_user_data.on_psk(
                &MosquittoClient {
                    client: event_data.client,
                },
                hint,
                identity,
                key,
                event_data.max_key_len as i32,
            )
        }

        #[no_mangle]
        extern "C" fn on_tick_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };
            let event_data: &mut mosquitto_evt_tick =
                unsafe { &mut *(event_data as *mut mosquitto_evt_tick) };

            user_data.external_user_data.on_tick(
                event_data.now_ns as i64,
                event_data.next_ns as i64,
                event_data.now_s as i32,
                event_data.next_s as i32,
            );
            0
        }

        #[no_mangle]
        extern "C" fn on_disconnect_trampoline(
            _event: c_int,
            event_data: *mut c_void,
            user_data: *mut c_void,
        ) -> c_int {
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };

            let event_data: &mut mosquitto_evt_disconnect =
                unsafe { &mut *(event_data as *mut mosquitto_evt_disconnect) };
            user_data.external_user_data.on_disconnect(
                &MosquittoClient {
                    client: event_data.client,
                },
                event_data.reason,
            );
            0
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_plugin_init(
            identifier: *mut c_void,
            user_data: *mut *mut c_void, // When this pointer is set, every other call will get this pointer as well. Only for v4 plugins?
            opts: *mut mosquitto_opt,
            opt_count: c_int,
        ) -> c_int {
            let opts = __from_ptr_and_size(opts, opt_count as _);
            println!("mosquitto_plugin_init {:?}", opts);

            let instance: $t = <$t>::init(opts);
            let instance = instance;
            println!("external_user_data addr {:?}", instance);
            let internal_user_data = InternalUserData {
                identifier,
                external_user_data: instance,
            };
            let internal_user_data = Box::new(internal_user_data);
            let instance_rawptr: *mut InternalUserData = Box::into_raw(internal_user_data);

            unsafe {
                *user_data = instance_rawptr as _;
            }

            unsafe {
                mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtReload as _,
                    Some(on_reload_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );

                mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtAclCheck as _,
                    Some(on_acl_check_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );

                mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtBasicAuth as _,
                    Some(on_basic_auth_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );

                let event_data = "$CONTROL";
                let cstr = &std::ffi::CString::new(event_data).unwrap();
                let bytes = cstr.as_bytes_with_nul();
                let topic = bytes.as_ptr() as *const c_void;
                // TODO the event_data parameter (4th param) has a meaning for the MOSQ_EVT_CONTROL callback
                // Something to do with the topic the control events are triggered on?
                //https://github.com/eclipse/mosquitto/blob/master/plugins/dynamic-security/plugin.c#L494
                mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtControl as _,
                    Some(on_control_trampoline),
                    topic,
                    instance_rawptr as _,
                );

                let res = mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtMessage as _,
                    Some(on_message_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );

                mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtPskKey as _,
                    Some(on_psk_key_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );

                mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtTick as _,
                    Some(on_tick_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );

                let res = mosquitto_callback_register(
                    identifier as _,
                    MosquittoPluginEvent::MosqEvtDisconnect as _,
                    Some(on_disconnect_trampoline),
                    std::ptr::null(),
                    instance_rawptr as _,
                );
            }

            Success.into()
        }

        #[no_mangle]
        extern "C" fn mosquitto_plugin_cleanup(
            user_data: *mut c_void,
            opts: *mut mosquitto_opt,
            opt_count: c_int,
        ) -> c_int {
            let opts = __from_ptr_and_size(opts, opt_count as _);
            let user_data: &mut InternalUserData =
                unsafe { &mut *(user_data as *mut InternalUserData) };

            unsafe {
                mosquitto_callback_unregister(
                    user_data.identifier as _,
                    MosquittoPluginEvent::MosqEvtDisconnect as _,
                    Some(on_disconnect_trampoline),
                    std::ptr::null(),
                );
            }
            println!("plugincleanup 2");

            drop(unsafe { Box::from_raw(user_data as *mut InternalUserData) });

            Success.into()
        }
    };
}
