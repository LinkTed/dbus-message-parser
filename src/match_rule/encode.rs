use super::MatchRule;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for MatchRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MatchRule::Type(r#type) => write!(f, "type={}", r#type),
            MatchRule::Sender(sender) => write!(f, "sender={}", sender),
            MatchRule::Interface(interface) => write!(f, "interface={}", interface),
            MatchRule::Member(member) => write!(f, "member={}", member),
            MatchRule::Path(path) => write!(f, "path={}", path),
            MatchRule::PathNamespace(path_namespace) => {
                write!(f, "path_namespace={}", path_namespace)
            }
            MatchRule::Destination(destination) => write!(f, "destination={}", destination),
            MatchRule::Arg(arg) => write!(f, "{}={}", arg.get_key(), arg),
            MatchRule::ArgPath(arg_path) => write!(f, "{}={}", arg_path.get_key(), arg_path),
            MatchRule::Arg0Namespace(arg0_namespace) => {
                write!(f, "arg0namespace={}", arg0_namespace)
            }
            MatchRule::Eavesdrop(eavesdrop) => {
                write!(f, "eavesdrop=")?;
                if *eavesdrop {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
        }
    }
}

impl MatchRule {
    pub fn encode(match_rules: &[MatchRule]) -> String {
        let mut iter = match_rules.iter();
        if let Some(match_rule) = iter.next() {
            let mut result = match_rule.to_string();
            for match_rule in iter {
                result.push(',');
                result.push_str(match_rule.to_string().as_str());
            }
            result
        } else {
            String::new()
        }
    }
}
