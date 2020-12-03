// Has to be included, to get the errors and success parameters that are used in the
// generate_dynamic_library macro invocation
use mosquitto_plugin::*;

// Some simple nonsense structure to use as an example
#[derive(Debug)]
pub struct Test {
    i: i32,
    s: String,
}

// Required trait implementation
impl MosquittoPlugin for Test {
    fn init(opts: std::collections::HashMap<&str, &str>) -> Self {
        // These are the strings provided after "auth_opt_<key> value" in the mosquitto.conf
        // only that they are provided on a hashmap form here
        let default = "hej";
        let topic = opts.get("topic").unwrap_or(&default);
        let level = opts.get("level").unwrap_or(&default);
        let level = level.parse().unwrap_or(0);

        Test {
            i: level,
            s: topic.to_string(),
        }
    }
    fn username_password(&mut self, u: &str, p: &str) -> Result<Success, Error> {
        // this will allow all username/password where the password is the username in reverse
        let rp: String = p.chars().rev().collect();
        if rp == u {
            Ok(Success)
        } else {
            Err(Error::Auth)
        }
    }

    fn acl_check(
        &mut self,
        level: i32,
        msg: MosquittoAclMessage,
    ) -> Result<Success, mosquitto_plugin::Error> {
        println!("allowed topic: {}", self.s);
        println!("topic: {}", msg.topic);
        println!("level requested: {}", level);

        // only the topic provided in the mosquitto.conf by the value auth_opt_topic <value> is
        // allowed, errors will not be reported to the clients though, they will only not be able
        // to send/receive messages and thus silently fail due to limitations in MQTT protocol
        if msg.topic == self.s {
            Ok(Success)
        } else {
            Err(Error::AclDenied)
        }
    }
}

// This generates the dynamic c bindings functions that are exported and usable by mosquitto
create_dynamic_library!(Test);
