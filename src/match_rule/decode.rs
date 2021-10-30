use super::{
    arg::Arg, arg_path::ArgPath, split::Split, unescape::unescape, MatchRule, MatchRuleError,
};
use crate::{
    message::MessageType,
    value::{Bus, Interface, Member, ObjectPath, UniqueConnectionName},
};
use std::convert::TryFrom;

impl TryFrom<&str> for MatchRule {
    type Error = MatchRuleError;

    fn try_from(match_rule: &str) -> Result<Self, Self::Error> {
        let (key, value) = match_rule
            .split_once('=')
            .ok_or(MatchRuleError::MissingEqual)?;

        if key == "type" {
            let r#type = unescape(value)?;
            let r#type = MessageType::try_from(r#type.as_str())?;
            Ok(MatchRule::Type(r#type))
        } else if key == "sender" {
            let sender = unescape(value)?;
            let sender = Bus::try_from(sender.as_str())?;
            Ok(MatchRule::Sender(sender))
        } else if key == "interface" {
            let interface = unescape(value)?;
            match Interface::try_from(interface.as_str()) {
                Ok(interface) => Ok(MatchRule::Interface(interface)),
                Err(e) => Err(MatchRuleError::InterfaceError(e)),
            }
        } else if key == "member" {
            let member = unescape(value)?;
            let member = Member::try_from(member.as_str())?;
            Ok(MatchRule::Member(member))
        } else if key == "path" {
            let path = unescape(value)?;
            match ObjectPath::try_from(path.as_str()) {
                Ok(path) => Ok(MatchRule::Path(path)),
                Err(e) => Err(MatchRuleError::PathError(e)),
            }
        } else if key == "path_namespace" {
            let path_namespace = unescape(value)?;
            match ObjectPath::try_from(path_namespace.as_str()) {
                Ok(path_namespace) => Ok(MatchRule::PathNamespace(path_namespace)),
                Err(e) => Err(MatchRuleError::PathErrorNamespace(e)),
            }
        } else if key == "destination" {
            let destination = unescape(value)?;
            let destination = UniqueConnectionName::try_from(destination.as_str())?;
            Ok(MatchRule::Destination(destination))
        } else if let Some(key_arg) = key.strip_prefix("arg") {
            if let Some(index) = key_arg.strip_suffix("path") {
                let index = index.parse::<usize>()?;
                let arg_path = unescape(value)?;
                match ObjectPath::try_from(arg_path.as_str()) {
                    Ok(arg_path) => Ok(MatchRule::ArgPath(ArgPath::from((index, arg_path)))),
                    Err(e) => Err(MatchRuleError::ArgPathError(e)),
                }
            } else if key_arg == "0namespace" {
                let arg0_namespace = unescape(value)?;
                match Interface::try_from(arg0_namespace.as_str()) {
                    Ok(arg0_namespace) => Ok(MatchRule::Arg0Namespace(arg0_namespace)),
                    Err(e) => Err(MatchRuleError::Arg0NamespaceError(e)),
                }
            } else {
                let index = key_arg.parse::<usize>()?;
                let arg = unescape(value)?;
                let arg = Arg::try_from((index, arg))?;
                Ok(MatchRule::Arg(arg))
            }
        } else if key == "eavesdrop" {
            let eavesdrop = unescape(value)?;
            match eavesdrop.as_str() {
                "true" => Ok(MatchRule::Eavesdrop(true)),
                "false" => Ok(MatchRule::Eavesdrop(false)),
                _ => Err(MatchRuleError::EavesdropUnknown),
            }
        } else {
            Err(MatchRuleError::KeyUnknown)
        }
    }
}

impl MatchRule {
    pub fn decode(match_rules: &str) -> Result<Vec<MatchRule>, MatchRuleError> {
        let mut result = Vec::new();
        for match_rule in Split::new(match_rules) {
            let match_rule = MatchRule::try_from(match_rule?)?;
            result.push(match_rule);
        }
        Ok(result)
    }
}
