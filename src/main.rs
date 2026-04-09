use clap::Parser;

mod cli;
mod config;
mod error;
mod types;

use cli::{Cli, Commands, ProviderCommands, TargetCommands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => config::init_config(),
        Commands::Provider(cmd) => handle_provider(cmd),
        Commands::Target(cmd) => handle_target(cmd),
        Commands::Use { provider, target } => {
            println!("Using provider '{}' for target '{}'", provider, target);
            Ok(())
        }
        Commands::Current => {
            println!("Showing current configuration");
            Ok(())
        }
        Commands::Status => {
            println!("Showing full status");
            Ok(())
        }
    }
}

fn handle_provider(cmd: ProviderCommands) -> anyhow::Result<()> {
    let mut config = config::load_config()?;

    match cmd {
        ProviderCommands::Add { name } => {
            if config.providers.contains_key(&name) {
                println!("Provider '{}' already exists", name);
                return Ok(());
            }

            // Interactive input
            use std::io::{self, Write};

            print!("API Key: ");
            io::stdout().flush()?;
            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key)?;

            print!("Base URL (optional): ");
            io::stdout().flush()?;
            let mut base_url = String::new();
            io::stdin().read_line(&mut base_url)?;

            print!("Model (optional): ");
            io::stdout().flush()?;
            let mut model = String::new();
            io::stdin().read_line(&mut model)?;

            let provider = types::Provider {
                api_key: api_key.trim().to_string(),
                base_url: if base_url.trim().is_empty() {
                    None
                } else {
                    Some(base_url.trim().to_string())
                },
                model: if model.trim().is_empty() {
                    None
                } else {
                    Some(model.trim().to_string())
                },
            };

            config.providers.insert(name.clone(), provider);
            config::save_config(&config)?;
            println!("Added provider '{}'", name);
            Ok(())
        }
        ProviderCommands::List => {
            if config.providers.is_empty() {
                println!("No providers configured");
            } else {
                println!("Configured providers:");
                for name in config.providers.keys() {
                    println!("  - {}", name);
                }
            }
            Ok(())
        }
        ProviderCommands::Remove { name } => {
            if config.providers.remove(&name).is_some() {
                // Remove from current mappings
                config.current.retain(|_, v| v != &name);
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

            // Validate target type
            let valid_types = ["claude-code", "cursor", "aider"];
            if !valid_types.contains(&target_type.as_str()) {
                println!("Invalid target type: {}", target_type);
                println!("Valid types: {}", valid_types.join(", "));
                return Ok(());
            }

            // Determine default config path based on target type
            let config_path = match target_type.as_str() {
                "claude-code" => dirs::home_dir()
                    .map(|h| h.join(".claude").join("CLAUDE.md"))
                    .map(|p| p.to_string_lossy().to_string()),
                "cursor" => dirs::home_dir()
                    .map(|h| h.join(".cursor").join("config.json"))
                    .map(|p| p.to_string_lossy().to_string()),
                "aider" => Some(".env".to_string()),
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
