#[macro_use]
extern crate honggfuzz;
use dbus_message_parser::match_rule::MatchRule;
use std::str::from_utf8;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(data) = from_utf8(data) {
                if let Ok(match_rules_1) = MatchRule::decode(data) {
                    let match_rules_1_string = MatchRule::encode(match_rules_1.as_ref());
                    match MatchRule::decode(match_rules_1_string.as_str()) {
                        Ok(match_rules_2) => {
                            if match_rules_1 != match_rules_2 {
                                panic!(
                                    "Messsage is not equal: {:?} != {:?}\n{:?} != {:?}",
                                    match_rules_1, match_rules_2, data, match_rules_1_string,
                                );
                            }
                        }
                        Err(e) => {
                            panic!(
                                "Could not decode DBus message:\n{:?}\n{:?}\n{:?}\n{:?}",
                                e, data, match_rules_1_string, match_rules_1
                            );
                        }
                    }
                }
            }
        });
    }
}
