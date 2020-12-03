#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// All the bindings provided by mosquitto_plugin.h
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
