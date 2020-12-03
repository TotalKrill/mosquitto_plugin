use mosquitto_plugin::*;

#[derive(Debug)]
pub struct Test {
    i: i32,
    s: String,
}

impl MosquittoPlugin for Test {
    fn init(opts: std::collections::HashMap<&str, &str>) -> Self {
        // These are the strings provided after "auth_opt_<key> value" in the mosquitto.conf
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
        if msg.topic == self.s {
            Ok(Success)
        } else {
            Err(Error::AclDenied)
        }
    }
}

// This generates the dynamic c bindings functions that are exported and usable by mosquitto
create_dynamic_library!(Test);
