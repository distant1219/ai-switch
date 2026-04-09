use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-switch")]
#[command(about = "Manage AI provider configurations for coding tools")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration
    Init,
    /// Manage providers
    #[command(subcommand)]
    Provider(ProviderCommands),
    /// Manage targets
    #[command(subcommand)]
    Target(TargetCommands),
    /// Use a provider configuration for a target
    Use {
        /// Provider profile name
        provider: String,
        /// Target tool name
        #[arg(short, long)]
        target: String,
    },
    /// Show current configuration
    Current,
    /// Show full status
    Status,
}

#[derive(Subcommand)]
pub enum ProviderCommands {
    /// Add a new provider configuration
    Add {
        /// Provider profile name
        name: String,
    },
    /// List all provider configurations
    List,
    /// Remove a provider configuration
    Remove {
        /// Provider profile name
        name: String,
    },
    /// Edit a provider configuration
    Edit {
        /// Provider profile name
        name: String,
    },
}

#[derive(Subcommand)]
pub enum TargetCommands {
    /// Add a new target tool
    Add {
        /// Target tool name
        name: String,
        /// Target type (claude-code, cursor, aider)
        #[arg(short, long)]
        target_type: String,
    },
    /// List all target tools
    List,
    /// Remove a target tool
    Remove {
        /// Target tool name
        name: String,
    },
}
