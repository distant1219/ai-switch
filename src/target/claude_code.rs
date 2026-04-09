use crate::target::TargetAdapter;
use crate::types::{ModelProfile, Provider, Target};
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub struct ClaudeCodeAdapter;

impl TargetAdapter for ClaudeCodeAdapter {
    fn apply(
        &self,
        provider: &Provider,
        target: &Target,
        profile_name: Option<&str>,
        model_profile: &ModelProfile,
    ) -> Result<()> {
        let config_path = shellexpand::tilde(&target.config_path).to_string();
        let path = Path::new(&config_path);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {:?}", parent))?;
        }

        // Read existing settings.json or create new
        let mut settings: Value = if path.exists() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read settings.json at {:?}", path))?;
            serde_json::from_str(&content)
                .unwrap_or(Value::Object(serde_json::Map::new().into()))
        } else {
            Value::Object(serde_json::Map::new().into())
        };

        // Ensure .env object exists
        if !settings.get("env").is_some() {
            settings["env"] = Value::Object(serde_json::Map::new().into());
        }
        let env = settings["env"].as_object_mut().unwrap();

        // Set API auth token
        env["ANTHROPIC_AUTH_TOKEN"] = Value::String(provider.api_key.clone());

        // Set base URL (profile overrides provider)
        let base_url = provider.base_url.as_deref().unwrap_or("https://api.anthropic.com");
        env["ANTHROPIC_BASE_URL"] = Value::String(base_url.to_string());

        // Set the active model
        env["ANTHROPIC_MODEL"] = Value::String(model_profile.model.clone());

        // Write all model profiles as ANTHROPIC_DEFAULT_*_MODEL env vars
        for (name, profile) in &provider.models {
            let env_key = match name.as_str() {
                "haiku" => "ANTHROPIC_DEFAULT_HAIKU_MODEL",
                "sonnet" => "ANTHROPIC_DEFAULT_SONNET_MODEL",
                "opus" => "ANTHROPIC_DEFAULT_OPUS_MODEL",
                _ => &format!("ANTHROPIC_DEFAULT_{}_MODEL", name.to_uppercase()),
            };
            env[env_key] = Value::String(profile.model.clone());
        }

        // Set top-level model alias (profile name)
        if let Some(name) = profile_name {
            settings["model"] = Value::String(name.to_string());
        }

        // Write back
        let content =
            serde_json::to_string_pretty(&settings).with_context(|| "Failed to serialize settings.json")?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write settings to {:?}", path))?;

        println!("Updated Claude Code settings at {}", config_path);
        Ok(())
    }
}
