use crate::target::TargetAdapter;
use crate::types::{ModelProfile, Provider, Target};
use anyhow::{Context, Result};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

pub struct ClaudeCodeAdapter;

impl TargetAdapter for ClaudeCodeAdapter {
    fn apply(
        &self,
        provider: &Provider,
        target: &Target,
        profile_name: Option<&str>,
        _model_profile: &ModelProfile,
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
                .unwrap_or_else(|_| Value::Object(Map::new()))
        } else {
            Value::Object(Map::new())
        };

        // Ensure .env object exists and is a map
        if !settings.is_object() {
            settings = Value::Object(Map::new());
        }
        let obj = settings.as_object_mut().unwrap();
        if !obj.contains_key("env") || !obj["env"].is_object() {
            obj.insert("env".to_string(), Value::Object(Map::new()));
        }

        let env = obj.get_mut("env").unwrap().as_object_mut().unwrap();

        // Helper to set a string value in the env map
        let set_env = |map: &mut Map<String, Value>, key: &str, val: String| {
            map.insert(key.to_string(), Value::String(val));
        };

        // Set API auth token
        set_env(env, "ANTHROPIC_AUTH_TOKEN", provider.api_key.clone());

        // Set base URL (profile overrides provider)
        let base_url = provider.base_url.as_deref().unwrap_or("https://api.anthropic.com");
        set_env(env, "ANTHROPIC_BASE_URL", base_url.to_string());

        // Set custom model option if configured
        if let Some(custom_opt) = &provider.custom_model_option {
            set_env(env, "ANTHROPIC_CUSTOM_MODEL_OPTION", custom_opt.clone());
        }

        // Write all model profiles as ANTHROPIC_DEFAULT_*_MODEL env vars
        for (name, profile) in &provider.models {
            let env_key = match name.as_str() {
                "haiku" => "ANTHROPIC_DEFAULT_HAIKU_MODEL",
                "sonnet" => "ANTHROPIC_DEFAULT_SONNET_MODEL",
                "opus" => "ANTHROPIC_DEFAULT_OPUS_MODEL",
                _ => &format!("ANTHROPIC_DEFAULT_{}_MODEL", name.to_uppercase()),
            };
            set_env(env, env_key, profile.model.clone());
        }

        // Set top-level model alias (profile name)
        if let Some(name) = profile_name {
            obj.insert("model".to_string(), Value::String(name.to_string()));
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
