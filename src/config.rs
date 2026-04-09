use crate::types::{Config, ModelProfile, Provider};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ai-switch")
        .join("config.toml")
}

pub fn ensure_config_dir() -> Result<PathBuf> {
    let path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ai-switch");
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Raw provider for deserialization - handles both old (model=) and new (models=) formats
#[derive(Debug, Deserialize)]
struct RawProvider {
    api_key: String,
    base_url: Option<String>,
    model: Option<String>,
    models: Option<HashMap<String, RawModelProfile>>,
    default_profile: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawModelProfile {
    model: String,
    display_name: Option<String>,
}

/// Raw config for deserialization with migration
#[derive(Debug, Deserialize)]
struct RawConfig {
    #[serde(default)]
    providers: HashMap<String, RawProvider>,
    #[serde(default)]
    targets: HashMap<String, crate::types::Target>,
    #[serde(default)]
    current: HashMap<String, String>,
    #[serde(default)]
    current_model: HashMap<String, String>,
}

fn migrate_provider(raw: RawProvider) -> Provider {
    let models: HashMap<String, ModelProfile> = raw
        .models
        .map(|m| {
            m.into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ModelProfile {
                            model: v.model,
                            display_name: v.display_name,
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    // If legacy single model field exists and no models map, migrate
    let (model, models, default_profile) = if models.is_empty() {
        match raw.model {
            Some(m) => {
                let mut migrated = HashMap::new();
                migrated.insert("default".to_string(), ModelProfile {
                    model: m.clone(),
                    display_name: None,
                });
                (None, migrated, None)
            }
            None => (None, HashMap::new(), raw.default_profile),
        }
    } else {
        (raw.model, models, raw.default_profile)
    };

    Provider {
        api_key: raw.api_key,
        base_url: raw.base_url,
        model,
        models,
        default_profile,
    }
}

fn migrate_config(raw: RawConfig) -> Config {
    Config {
        providers: raw
            .providers
            .into_iter()
            .map(|(k, v)| (k, migrate_provider(v)))
            .collect(),
        targets: raw.targets,
        current: raw.current,
        current_model: raw.current_model,
    }
}

pub fn load_config() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read config from {:?}", path))?;
    let raw: RawConfig =
        toml::from_str(&content).with_context(|| "Failed to parse config.toml")?;
    Ok(migrate_config(raw))
}

pub fn save_config(config: &Config) -> Result<()> {
    ensure_config_dir()?;
    let path = config_path();
    let content =
        toml::to_string_pretty(config).with_context(|| "Failed to serialize config")?;
    fs::write(&path, content)
        .with_context(|| format!("Failed to write config to {:?}", path))?;
    Ok(())
}

pub fn init_config() -> Result<()> {
    ensure_config_dir()?;
    let path = config_path();
    if !path.exists() {
        let config = Config::default();
        save_config(&config)?;
        println!("Initialized config at {:?}", path);
    } else {
        println!("Config already exists at {:?}", path);
    }
    Ok(())
}
