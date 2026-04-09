use clap::Parser;

mod cli;
mod config;
mod core;
mod error;
mod target;
mod types;

use cli::{Cli, Commands, ModelCommands, ProviderCommands, TargetCommands};
use types::ModelProfile;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => config::init_config(),
        Commands::Provider(cmd) => handle_provider(cmd),
        Commands::Target(cmd) => handle_target(cmd),
        Commands::Use { provider, target, model } => {
            core::use_provider(&provider, &target, model.as_deref())
        }
        Commands::Current => core::show_current(),
        Commands::Status => core::show_status(),
    }
}

fn handle_provider(cmd: ProviderCommands) -> anyhow::Result<()> {
    match cmd {
        ProviderCommands::Add { name } => {
            let mut config = config::load_config()?;
            if config.providers.contains_key(&name) {
                println!("Provider '{}' already exists", name);
                return Ok(());
            }

            use std::io::{self, Write};

            print!("API Key: ");
            io::stdout().flush()?;
            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key)?;

            print!("Base URL (optional): ");
            io::stdout().flush()?;
            let mut base_url = String::new();
            io::stdin().read_line(&mut base_url)?;

            let mut provider = types::Provider {
                api_key: api_key.trim().to_string(),
                base_url: if base_url.trim().is_empty() {
                    None
                } else {
                    Some(base_url.trim().to_string())
                },
                model: None,
                models: std::collections::HashMap::new(),
                default_profile: None,
            };

            // Ask if user wants to add Claude model profiles
            print!("\nAdd Claude model presets (haiku/sonnet/opus)? [y/N]: ");
            io::stdout().flush()?;
            let mut add_models = String::new();
            io::stdin().read_line(&mut add_models)?;

            if add_models.trim().to_lowercase() == "y" {
                // Prompt for each model
                print!("Haiku model ID [claude-haiku-4-20250514]: ");
                io::stdout().flush()?;
                let mut haiku = String::new();
                io::stdin().read_line(&mut haiku)?;
                let haiku = haiku.trim();
                let haiku_model = if haiku.is_empty() { "claude-haiku-4-20250514" } else { haiku };

                print!("Sonnet model ID [claude-sonnet-4-20250514]: ");
                io::stdout().flush()?;
                let mut sonnet = String::new();
                io::stdin().read_line(&mut sonnet)?;
                let sonnet = sonnet.trim();
                let sonnet_model = if sonnet.is_empty() { "claude-sonnet-4-20250514" } else { sonnet };

                print!("Opus model ID [claude-opus-4-20250514]: ");
                io::stdout().flush()?;
                let mut opus = String::new();
                io::stdin().read_line(&mut opus)?;
                let opus = opus.trim();
                let opus_model = if opus.is_empty() { "claude-opus-4-20250514" } else { opus };

                // Add models to provider
                provider.models.insert("haiku".to_string(), types::ModelProfile {
                    model: haiku_model.to_string(),
                    display_name: Some("Haiku (fast)".to_string()),
                });
                provider.models.insert("sonnet".to_string(), types::ModelProfile {
                    model: sonnet_model.to_string(),
                    display_name: Some("Sonnet (balanced)".to_string()),
                });
                provider.models.insert("opus".to_string(), types::ModelProfile {
                    model: opus_model.to_string(),
                    display_name: Some("Opus (smart)".to_string()),
                });

                // Set sonnet as default
                provider.default_profile = Some("sonnet".to_string());

                println!("\nAdded model profiles: haiku, sonnet, opus (default: sonnet)");
            }

            config.providers.insert(name.clone(), provider);
            config::save_config(&config)?;
            println!("\nAdded provider '{}'", name);
            if !config.providers[&name].has_profiles() {
                println!("Tip: Use 'ai-switch provider model add {} <profile> --model <id>' to add model profiles", name);
            }
            Ok(())
        }
        ProviderCommands::List => {
            let config = config::load_config()?;
            if config.providers.is_empty() {
                println!("No providers configured");
            } else {
                println!("Configured providers:");
                for (name, provider) in &config.providers {
                    if provider.has_profiles() {
                        let profiles: Vec<&str> = provider.models.keys().map(|s| s.as_str()).collect();
                        let default = provider.default_profile.as_deref().unwrap_or("(none)");
                        println!("  - {} [models: {}] (default: {})", name, profiles.join(", "), default);
                    } else {
                        let model = provider.model.as_deref().unwrap_or("(none)");
                        println!("  - {} (model: {})", name, model);
                    }
                }
            }
            Ok(())
        }
        ProviderCommands::Remove { name } => {
            let mut config = config::load_config()?;
            if config.providers.remove(&name).is_some() {
                config.current.retain(|_, v| v != &name);
                config.current_model.remove(&name);
                config::save_config(&config)?;
                println!("Removed provider '{}'", name);
            } else {
                println!("Provider '{}' not found", name);
            }
            Ok(())
        }
        ProviderCommands::Edit { name } => {
            println!("Edit provider '{}' (not implemented yet)", name);
            Ok(())
        }
        ProviderCommands::Model(model_cmd) => handle_model(model_cmd),
    }
}

fn handle_model(cmd: ModelCommands) -> anyhow::Result<()> {
    match cmd {
        ModelCommands::Add { provider, profile, model, display_name, default } => {
            let mut config = config::load_config()?;
            let prov = config.providers.get_mut(&provider)
                .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", provider))?;

            if prov.models.contains_key(&profile) {
                println!("Profile '{}' already exists in provider '{}'", profile, provider);
                return Ok(());
            }

            prov.models.insert(profile.clone(), ModelProfile {
                model,
                display_name,
            });

            if default || prov.default_profile.is_none() {
                prov.default_profile = Some(profile.clone());
            }

            config::save_config(&config)?;
            println!("Added profile '{}' to provider '{}'", profile, provider);
            Ok(())
        }
        ModelCommands::List { provider } => {
            let config = config::load_config()?;
            let prov = config.providers.get(&provider)
                .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", provider))?;

            if prov.models.is_empty() {
                println!("No model profiles configured for '{}'", provider);
                if let Some(model) = &prov.model {
                    println!("  Legacy model: {}", model);
                }
            } else {
                let default = prov.default_profile.as_deref().unwrap_or("(none)");
                println!("Model profiles for '{}' (default: {}):", provider, default);
                for (name, profile) in &prov.models {
                    let display = profile.display_name.as_deref().unwrap_or("");
                    let marker = if Some(name) == prov.default_profile.as_ref() { " *" } else { "" };
                    if display.is_empty() {
                        println!("  - {} ({}){}", name, profile.model, marker);
                    } else {
                        println!("  - {} ({}) - {}{}", name, profile.model, display, marker);
                    }
                }
            }
            Ok(())
        }
        ModelCommands::Remove { provider, profile } => {
            let mut config = config::load_config()?;
            let prov = config.providers.get_mut(&provider)
                .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", provider))?;

            if prov.models.remove(&profile).is_some() {
                // If removed profile was default, try to set a new default
                if prov.default_profile.as_deref() == Some(&profile) {
                    prov.default_profile = prov.models.keys().next().cloned();
                }
                config::save_config(&config)?;
                println!("Removed profile '{}' from provider '{}'", profile, provider);
            } else {
                println!("Profile '{}' not found in provider '{}'", profile, provider);
            }
            Ok(())
        }
        ModelCommands::SetDefault { provider, profile } => {
            let mut config = config::load_config()?;
            let prov = config.providers.get_mut(&provider)
                .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found", provider))?;

            if !prov.models.contains_key(&profile) {
                println!("Profile '{}' not found in provider '{}'", profile, provider);
                return Ok(());
            }

            prov.default_profile = Some(profile.clone());
            config::save_config(&config)?;
            println!("Set '{}' as default profile for provider '{}'", profile, provider);
            Ok(())
        }
    }
}

fn handle_target(cmd: TargetCommands) -> anyhow::Result<()> {
    let mut config = config::load_config()?;

    match cmd {
        TargetCommands::Add { name, target_type } => {
            if config.targets.contains_key(&name) {
                println!("Target '{}' already exists", name);
                return Ok(());
            }

            let valid_types = ["claude-code", "cursor", "aider"];
            if !valid_types.contains(&target_type.as_str()) {
                println!("Invalid target type: {}", target_type);
                println!("Valid types: {}", valid_types.join(", "));
                return Ok(());
            }

            let config_path = match target_type.as_str() {
                "claude-code" => dirs::home_dir()
                    .map(|h| h.join(".claude").join("settings.json"))
                    .map(|p| p.to_string_lossy().to_string()),
                "cursor" => dirs::home_dir()
                    .map(|h| h.join(".config").join("Cursor").join("User").join("settings.json"))
                    .map(|p| p.to_string_lossy().to_string()),
                "aider" => Some(".aider.conf.yml".to_string()),
                _ => None,
            };

            let config_path = match config_path {
                Some(path) => path,
                None => {
                    use std::io::{self, Write};
                    print!("Config path: ");
                    io::stdout().flush()?;
                    let mut path = String::new();
                    io::stdin().read_line(&mut path)?;
                    path.trim().to_string()
                }
            };

            let target = types::Target {
                target_type,
                config_path,
            };

            config.targets.insert(name.clone(), target);
            config::save_config(&config)?;
            println!("Added target '{}'", name);
            Ok(())
        }
        TargetCommands::List => {
            if config.targets.is_empty() {
                println!("No targets configured");
            } else {
                println!("Configured targets:");
                for (name, target) in &config.targets {
                    println!("  - {} (type: {}, path: {})", name, target.target_type, target.config_path);
                }
            }
            Ok(())
        }
        TargetCommands::Remove { name } => {
            if config.targets.remove(&name).is_some() {
                config.current.remove(&name);
                config::save_config(&config)?;
                println!("Removed target '{}'", name);
            } else {
                println!("Target '{}' not found", name);
            }
            Ok(())
        }
    }
}
