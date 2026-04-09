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
    match cmd {
        TargetCommands::Add { name, target_type } => {
            println!("Adding target: {} (type: {})", name, target_type);
            Ok(())
        }
        TargetCommands::List => {
            println!("Listing targets");
            Ok(())
        }
        TargetCommands::Remove { name } => {
            println!("Removing target: {}", name);
            Ok(())
        }
    }
}
