use crate::config;
use crate::types::{Config, Provider, Target};
use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn use_provider(provider_name: &str, target_name: &str) -> Result<()> {
    let mut config = config::load_config()?;

    // Validate provider exists
    let provider = config
        .providers
        .get(provider_name)
        .with_context(|| format!("Provider '{}' not found", provider_name))?
        .clone();

    // Validate target exists
    let target = config
        .targets
        .get(target_name)
        .with_context(|| format!("Target '{}' not found", target_name))?
        .clone();

    // Apply configuration to target
    apply_to_target(&provider, &target)?;

    // Update current mapping
    config.current.insert(target_name.to_string(), provider_name.to_string());
    config::save_config(&config)?;

    println!("Applied provider '{}' to target '{}'", provider_name, target_name);
    Ok(())
}

fn apply_to_target(provider: &Provider, target: &Target) -> Result<()> {
    match target.target_type.as_str() {
        "claude-code" => apply_to_claude_code(provider, target),
        "cursor" => apply_to_cursor(provider, target),
        "aider" => apply_to_aider(provider, target),
        _ => Err(anyhow::anyhow!("Unsupported target type: {}", target.target_type)),
    }
}

fn apply_to_claude_code(provider: &Provider, target: &Target) -> Result<()> {
    // For now, just print what would be done
    println!("Would update {} with:", target.config_path);
    println!("  API Key: {}...", &provider.api_key[..8.min(provider.api_key.len())]);
    if let Some(url) = &provider.base_url {
        println!("  Base URL: {}", url);
    }
    if let Some(model) = &provider.model {
        println!("  Model: {}", model);
    }
    Ok(())
}

fn apply_to_cursor(_provider: &Provider, _target: &Target) -> Result<()> {
    println!("Cursor support not yet implemented");
    Ok(())
}

fn apply_to_aider(_provider: &Provider, _target: &Target) -> Result<()> {
    println!("Aider support not yet implemented");
    Ok(())
}

pub fn show_current() -> Result<()> {
    let config = config::load_config()?;

    if config.current.is_empty() {
        println!("No active configurations");
        return Ok(());
    }

    println!("Current configurations:");
    for (target_name, provider_name) in &config.current {
        println!("  {} -> {}", target_name, provider_name);
    }

    Ok(())
}

pub fn show_status() -> Result<()> {
    let config = config::load_config()?;

    println!("=== AI Switch Status ===\n");

    println!("Providers:");
    if config.providers.is_empty() {
        println!("  (none)");
    } else {
        for name in config.providers.keys() {
            println!("  - {}", name);
        }
    }

    println!("\nTargets:");
    if config.targets.is_empty() {
        println!("  (none)");
    } else {
        for (name, target) in &config.targets {
            println!("  - {} (type: {})", name, target.target_type);
        }
    }

    println!("\nCurrent Mappings:");
    if config.current.is_empty() {
        println!("  (none)");
    } else {
        for (target, provider) in &config.current {
            println!("  {} -> {}", target, provider);
        }
    }

    Ok(())
}
