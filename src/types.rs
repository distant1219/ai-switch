use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub target_type: String,
    pub config_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub providers: HashMap<String, Provider>,
    pub targets: HashMap<String, Target>,
    pub current: HashMap<String, String>, // target_name -> provider_name
}

#[derive(Debug, Clone)]
pub struct ProviderType;

impl ProviderType {
    pub const OPENAI: &'static str = "openai";
    pub const ANTHROPIC: &'static str = "anthropic";
    pub const ZHIPU: &'static str = "zhipu";
    pub const GOOGLE: &'static str = "google";
}

#[derive(Debug, Clone)]
pub struct TargetType;

impl TargetType {
    pub const CLAUDE_CODE: &'static str = "claude-code";
    pub const CURSOR: &'static str = "cursor";
    pub const AIDER: &'static str = "aider";
}
