use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelProfile {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub models: HashMap<String, ModelProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_option: Option<String>,
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
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub current_model: HashMap<String, String>, // provider_name -> profile_name
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

impl Provider {
    /// Resolve a model profile by name, falling back to default or single profile.
    pub fn resolve_model(&self, profile_name: Option<&str>) -> anyhow::Result<(String, ModelProfile)> {
        if !self.models.is_empty() {
            if let Some(name) = profile_name {
                let profile = self
                    .models
                    .get(name)
                    .ok_or_else(|| anyhow::anyhow!("Model profile '{}' not found in provider", name))?;
                return Ok((name.to_string(), profile.clone()));
            }
            if let Some(default) = &self.default_profile {
                if let Some(profile) = self.models.get(default) {
                    return Ok((default.clone(), profile.clone()));
                }
            }
            if self.models.len() == 1 {
                let (k, v) = self.models.iter().next().unwrap();
                return Ok((k.clone(), v.clone()));
            }
            anyhow::bail!("No default model profile configured. Use --model to specify one.");
        }
        // Legacy: return model directly with empty profile name
        if let Some(model) = &self.model {
            return Ok(("".to_string(), ModelProfile {
                model: model.clone(),
                display_name: None,
            }));
        }
        anyhow::bail!("No model configured for this provider");
    }

    pub fn has_profiles(&self) -> bool {
        !self.models.is_empty()
    }
}
