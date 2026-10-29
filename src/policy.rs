//! Subset of the #38 / iouring-lab JSON policy shape.

use serde::Deserialize;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("defaultAction must be allow|deny")]
    BadDefault,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawPolicy {
    pub default_action: String,
    #[serde(default)]
    pub ops: Vec<String>,
    #[serde(default)]
    pub deny_ops: Vec<String>,
    #[serde(default)]
    pub socket_families: Vec<String>,
    #[serde(default)]
    pub denied_sqe_flags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub default_action: String,
    pub ops: Vec<String>,
    pub deny_ops: Vec<String>,
    pub socket_families: Vec<String>,
    pub denied_sqe_flags: Vec<String>,
}

impl Policy {
    pub fn parse_json(s: &str) -> Result<Self, PolicyError> {
        let raw: RawPolicy = serde_json::from_str(s)?;
        let d = raw.default_action.to_ascii_lowercase();
        if d != "allow" && d != "deny" {
            return Err(PolicyError::BadDefault);
        }
        Ok(Policy {
            default_action: d,
            ops: raw.ops,
            deny_ops: raw.deny_ops,
            socket_families: raw.socket_families,
            denied_sqe_flags: raw.denied_sqe_flags,
        })
    }

    pub fn denied_opcode_names(&self) -> BTreeSet<String> {
        let mut d: BTreeSet<String> = self.deny_ops.iter().cloned().collect();
        if self.default_action == "deny" {
            // Known residual ops often denied by default lab profile.
            for name in [
                "IORING_OP_MSG_RING",
                "IORING_OP_URING_CMD",
                "IORING_OP_PROVIDE_BUFFERS",
                "IORING_OP_REMOVE_BUFFERS",
            ] {
                if !self.ops.iter().any(|o| o == name) {
                    d.insert(name.into());
                }
            }
        }
        d
    }

    pub fn summary_line(&self) -> String {
        format!(
            "default={} allow_ops={} deny_ops={} families={} denied_flags={}",
            self.default_action,
            self.ops.len(),
            self.deny_ops.len(),
            self.socket_families.len(),
            self.denied_sqe_flags.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_default_fixture() {
        let p = Policy::parse_json(include_str!("../fixtures/default.json")).unwrap();
        assert_eq!(p.default_action, "deny");
        assert_eq!(p.ops.len(), 38);
        assert!(p.denied_opcode_names().contains("IORING_OP_MSG_RING"));
    }
}
