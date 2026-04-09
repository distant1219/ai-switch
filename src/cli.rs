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
        /// Model profile name (for providers with multiple models)
        #[arg(short, long)]
        model: Option<String>,
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
    /// Manage model profiles for a provider
    #[command(subcommand)]
    Model(ModelCommands),
}

#[derive(Subcommand)]
pub enum ModelCommands {
    /// Add a model profile to a provider
    Add {
        /// Provider name
        provider: String,
        /// Profile name (e.g., haiku, sonnet, opus)
        profile: String,
        /// Model ID (e.g., claude-sonnet-4-20250514)
        #[arg(short, long)]
        model: String,
        /// Display name for the profile
        #[arg(short, long)]
        display_name: Option<String>,
        /// Set as default profile
        #[arg(long)]
        default: bool,
    },
    /// List model profiles for a provider
    List {
        /// Provider name
        provider: String,
    },
    /// Remove a model profile from a provider
    Remove {
        /// Provider name
        provider: String,
        /// Profile name
        profile: String,
    },
    /// Set the default model profile for a provider
    SetDefault {
        /// Provider name
        provider: String,
        /// Profile name
        profile: String,
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
