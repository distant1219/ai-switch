use crate::config;
use crate::target::get_adapter;
use crate::types::{Config, Provider, Target};
use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn use_provider(provider_name: &str, target_name: &str) -> Result<()> {
    let mut config = config::load_config()?;

    let provider = config
        .providers
        .get(provider_name)
        .with_context(|| format!("Provider '{}' not found", provider_name))?
        .clone();

    let target = config
        .targets
        .get(target_name)
        .with_context(|| format!("Target '{}' not found", target_name))?
        .clone();

    // Use target adapter to apply configuration
    if let Some(adapter) = get_adapter(&target.target_type) {
        adapter.apply(&provider, &target)?;
    } else {
        return Err(anyhow::anyhow!(
            "No adapter found for target type: {}",
            target.target_type
        ));
    }

    config.current.insert(target_name.to_string(), provider_name.to_string());
    config::save_config(&config)?;

    println!("Applied provider '{}' to target '{}'", provider_name, target_name);
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
