# Example ACL

Simple example that allows only password/username combos where the password is reversed (and no credentials as well, since those do not invoke ACL calls, and thus needs to be configured in a
mosquitto configuration)

It also only allows messages on the topic specified in the mosquitto config as auth_opt_topic

see the provided mosquitto.conf for details
