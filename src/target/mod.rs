pub mod claude_code;

use crate::types::{ModelProfile, Provider, Target};
use anyhow::Result;

pub trait TargetAdapter {
    fn apply(
        &self,
        provider: &Provider,
        target: &Target,
        profile_name: Option<&str>,
        model_profile: &ModelProfile,
    ) -> Result<()>;
}

pub fn get_adapter(target_type: &str) -> Option<Box<dyn TargetAdapter>> {
    match target_type {
        "claude-code" => Some(Box::new(claude_code::ClaudeCodeAdapter)),
        _ => None,
    }
}
