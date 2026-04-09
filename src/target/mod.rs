pub mod claude_code;

use crate::types::{Provider, Target};
use anyhow::Result;

pub trait TargetAdapter {
    fn apply(&self, provider: &Provider, target: &Target) -> Result<()>;
}

pub fn get_adapter(target_type: &str) -> Option<Box<dyn TargetAdapter>> {
    match target_type {
        "claude-code" => Some(Box::new(claude_code::ClaudeCodeAdapter)),
        _ => None,
    }
}
