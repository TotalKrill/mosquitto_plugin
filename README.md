[![CI](https://github.com/TotalKrill/mosquitto_plugin/workflows/ci/badge.svg)](https://github.com/TotalKrill/mosquitto_plugin/actions)

# Mosquitto Plugin

A simple way to generate ACL and PASSWORD plugins for usage with the mosquitto broker.

requires that mosquitto_plugin.h mosquitto.h files are installed on the system, on linux systems
this is usually achieved through the mosquitto-dev packages. Not tested on windows

The optional functions are not implemented here.

## Supported

    - ease of access to write own mosquitto plugins
    - auth_opt_<key> value in the mosquitto_conf
    - mutable access to the structure between calls
    - ACL implementations
    - username/password implementatations

## Example usage

There is an example usage in the github repo under "example-acl" folder
