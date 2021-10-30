use super::MatchRule;
use crate::{
    message::Message,
    value::{Bus, Value},
};

fn match_namespace<const P: char>(namespace: &str, value: &str) -> bool {
    if let Some(value) = value.strip_prefix(namespace) {
        value.starts_with(P) || value.is_empty()
    } else {
        false
    }
}

fn get_str_from_string_or_object_path(value: &Value) -> Option<&str> {
    match value {
        Value::String(string) => Some(string),
        Value::ObjectPath(object_path) => Some(object_path.as_ref()),
        _ => None,
    }
}

impl MatchRule {
    pub fn matching_rule(&self, message: &Message) -> bool {
        match self {
            MatchRule::Type(r#type) => r#type == &message.get_type(),
            MatchRule::Sender(sender_1) => {
                if let Some(sender_2) = message.get_sender() {
                    sender_1 == sender_2
                } else {
                    false
                }
            }
            MatchRule::Interface(interface_1) => {
                if let Some(interface_2) = message.get_interface() {
                    interface_1 == interface_2
                } else {
                    false
                }
            }
            MatchRule::Member(member_1) => {
                if let Some(member_2) = message.get_member() {
                    member_1 == member_2
                } else {
                    false
                }
            }
            MatchRule::Path(path_1) => {
                if let Some(path_2) = message.get_path() {
                    path_1 == path_2
                } else {
                    false
                }
            }
            MatchRule::PathNamespace(path_namespace) => {
                if let Some(path) = message.get_path() {
                    match_namespace::<'/'>(path_namespace.as_ref(), path.as_ref())
                } else {
                    false
                }
            }
            MatchRule::Destination(destination_1) => {
                if let Some(destination_2) = message.get_destination() {
                    match destination_2 {
                        Bus::WellKnownBusName(_) => false,
                        Bus::UniqueConnectionName(destination_2) => destination_1 == destination_2,
                    }
                } else {
                    false
                }
            }
            MatchRule::Arg(arg) => {
                let body = message.get_body();
                if let Some(Value::String(value)) = body.get(arg.get_index()) {
                    arg.get_value() == value
                } else {
                    false
                }
            }
            MatchRule::ArgPath(arg_path) => {
                let body = message.get_body();
                if let Some(value) = body.get(arg_path.get_index()) {
                    if let Some(string) = get_str_from_string_or_object_path(value) {
                        match_namespace::<'/'>(arg_path.get_value().as_ref(), string)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            MatchRule::Arg0Namespace(arg0_namespace) => {
                let body = message.get_body();
                if let Some(Value::String(arg0)) = body.get(0) {
                    match_namespace::<'.'>(arg0_namespace.as_ref(), arg0)
                } else {
                    false
                }
            }
            MatchRule::Eavesdrop(_) => true,
        }
    }

    pub fn matching_rules(match_rules: &[MatchRule], message: &Message) -> bool {
        for match_rule in match_rules.iter() {
            if !match_rule.matching_rule(message) {
                return false;
            }
        }
        true
    }
}
