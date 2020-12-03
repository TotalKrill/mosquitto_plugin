// This generates the dynamic c bindings functions that are exported and usable by mosquitto, as
// well as allocating memory for the structure and handles recreating this from raw memory, to
// allow the generated plugin to use member functions, and thus have mutable state
#[macro_export]
macro_rules! create_dynamic_library {
    ($t:ty) => {
        use std::os::raw::c_char as Char;
        use std::os::raw::c_int as Int;
        fn __assert_sync()
        where
            $t: MosquittoPlugin,
        {
        }

        use std::os::raw::c_void as Void;

        #[no_mangle]
        pub extern "C" fn mosquitto_auth_plugin_version() -> isize {
            mosquitto_dev::MOSQ_AUTH_PLUGIN_VERSION as isize
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_auth_plugin_init(
            user_data: *mut *mut Void, // When this pointer is set, every other call will get this pointer as well
            opts: *mut mosquitto_opt,
            opt_count: Int,
        ) -> Int {
            let opts = __from_ptr_and_size(opts, opt_count as _);
            let instance = <$t>::init(opts);
            let instance = Box::new(instance);
            let instance_rawptr: *mut $t = Box::into_raw(instance);

            unsafe {
                *user_data = instance_rawptr as _;
            }

            Success.into()
        }

        #[no_mangle]
        extern "C" fn mosquitto_auth_plugin_cleanup(
            user_data: *mut Void,
            opts: *mut mosquitto_opt,
            opt_count: Int,
        ) -> Int {
            let opts = __from_ptr_and_size(opts, opt_count as _);
            drop(unsafe { Box::from_raw(user_data as *mut $t) });
            Success.into()
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_auth_security_init(
            user_data: *mut Void,
            opts: *mut mosquitto_opt,
            opt_count: Int,
            reload: bool,
        ) -> Int {
            let me: &mut $t = unsafe { &mut *(user_data as *mut $t) };
            let opts = __from_ptr_and_size(opts, opt_count as _);

            Success.into()
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_auth_security_cleanup(
            user_data: *mut Void,
            opts: *mut mosquitto_opt,
            opt_count: Int,
            reload: bool,
        ) -> Int {
            let me: &mut $t = unsafe { &mut *(user_data as *mut $t) };
            let opts = __from_ptr_and_size(opts, opt_count as _);

            Success.into()
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_auth_acl_check(
            user_data: *mut Void,
            access: Int,
            client: *mut mosquitto,
            msg: *const mosquitto_acl_msg,
        ) -> Int {
            //let level: AccessLevel = access.into();
            let level = access;
            let me: &mut $t = unsafe { &mut *(user_data as *mut $t) };

            let msg: &mosquitto_acl_msg = unsafe {
                let msg = msg.as_ref().unwrap();
                msg
            };
            let topic: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(msg.topic);
                c_str.to_str().unwrap()
            };

            let payload: &[u8] = unsafe {
                std::slice::from_raw_parts(msg.payload as *const u8, msg.payloadlen as usize)
            };

            let retain = msg.retain;
            //let qos = QoS::from_num(msg.qos);
            let qos = msg.qos;

            let msg = MosquittoAclMessage {
                topic,
                payload,
                qos,
                retain,
            };

            let res = me.acl_check(level, msg);

            match res {
                Ok(s) => s.into(),
                Err(e) => e.into(),
            }
        }

        #[no_mangle]
        pub extern "C" fn mosquitto_auth_unpwd_check(
            user_data: *mut Void,
            client: *mut mosquitto,
            username: *const Char,
            password: *const Char,
        ) -> Int {
            let me: &mut $t = unsafe { &mut *(user_data as *mut $t) };

            let username: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(username);
                c_str.to_str().unwrap()
            };

            let password: &str = unsafe {
                let c_str = std::ffi::CStr::from_ptr(password);
                c_str.to_str().unwrap()
            };

            let res = me.username_password(username, password);

            match res {
                Ok(s) => s.into(),
                Err(e) => e.into(),
            }
        }
    };
}
