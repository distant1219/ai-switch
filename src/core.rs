use crate::config;
use crate::target::get_adapter;
use anyhow::{Context, Result};

pub fn use_provider(provider_name: &str, target_name: &str, profile_name: Option<&str>) -> Result<()> {
    let mut config = config::load_config()?;

    let provider = config
        .providers
        .get(provider_name)
        .with_context(|| format!("Provider '{}' not found", provider_name))?;

    let target = config
        .targets
        .get(target_name)
        .with_context(|| format!("Target '{}' not found", target_name))?;

    // Resolve the model profile
    let (resolved_name, model_profile) = provider
        .resolve_model(profile_name)
        .with_context(|| format!("Failed to resolve model for provider '{}'", provider_name))?;

    let resolved_name_opt = if resolved_name.is_empty() {
        None
    } else {
        Some(resolved_name.clone())
    };

    // Use target adapter to apply configuration
    if let Some(adapter) = get_adapter(&target.target_type) {
        adapter.apply(provider, target, resolved_name_opt.as_deref(), &model_profile)?;
    } else {
        return Err(anyhow::anyhow!(
            "No adapter found for target type: {}",
            target.target_type
        ));
    }

    config.current.insert(target_name.to_string(), provider_name.to_string());
    if provider.has_profiles() {
        config.current_model.insert(
            provider_name.to_string(),
            resolved_name.clone(),
        );
    }
    config::save_config(&config)?;

    let model_desc = if resolved_name.is_empty() {
        &model_profile.model
    } else {
        &resolved_name
    };
    println!(
        "Applied provider '{}' (model: {}) to target '{}'",
        provider_name, model_desc, target_name
    );
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
        let model_info = config.current_model.get(provider_name);
        if let Some(model) = model_info {
            println!("  {} -> {} [{}]", target_name, provider_name, model);
        } else {
            println!("  {} -> {}", target_name, provider_name);
        }
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
        for (name, provider) in &config.providers {
            if provider.has_profiles() {
                let default = provider
                    .default_profile
                    .as_deref()
                    .unwrap_or("(no default)");
                let profiles: Vec<&str> = provider.models.keys().map(|s| s.as_str()).collect();
                println!(
                    "  - {} [models: {}] (default: {})",
                    name,
                    profiles.join(", "),
                    default
                );
            } else {
                let model = provider.model.as_deref().unwrap_or("(none)");
                println!("  - {} (model: {})", name, model);
            }
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
            let model = config.current_model.get(provider);
            if let Some(m) = model {
                println!("  {} -> {} [{}]", target, provider, m);
            } else {
                println!("  {} -> {}", target, provider);
            }
        }
    }

    Ok(())
}
