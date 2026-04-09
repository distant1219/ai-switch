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
    match cmd {
        ProviderCommands::Add { name } => {
            println!("Adding provider: {}", name);
            Ok(())
        }
        ProviderCommands::List => {
            println!("Listing providers");
            Ok(())
        }
        ProviderCommands::Remove { name } => {
            println!("Removing provider: {}", name);
            Ok(())
        }
        ProviderCommands::Edit { name } => {
            println!("Editing provider: {}", name);
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
