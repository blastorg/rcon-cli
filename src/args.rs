use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RconArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,

    /// Flag dictating whether to use the saved default server or pick from list of servers
    #[clap(long, short, action, verbatim_doc_comment)]
    pub default_server: bool,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Manage saved server configurations
    Server(ServerManagementCommands),

    /// Perform actions on a steam server
    Action(RconActions),

    /// Generate shell completions file
    ShellCompletion(ShellCompletion),
}

#[derive(Debug, Args)]
pub struct ServerManagementCommands {
    #[clap(subcommand)]
    pub cmd: ServerManagementSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum ServerManagementSubcommands {
    /// List all saved server configurations
    List,
    /// Add a new server configuration
    Add,
    /// Set default server from saved Servers
    SetDefault,
    /// Edit a server configuration
    Edit,
    /// Remove a server from saved Servers
    Remove,
    /// Clear all saved server configurations
    Clear,
}

#[derive(Debug, Args)]
pub struct RconActions {
    #[clap(subcommand)]
    pub cmd: RconActionsSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum RconActionsSubcommands {
    /// Execute a command on the selected server with dynamic prompt
    Shell,
    Exec(ExecSubCommand),
}
#[derive(Debug, Args)]
pub struct ExecSubCommand {
    /// Command to execute
    pub command: String,
}

#[derive(Debug, Args)]
pub struct ShellCompletion {
    /// Path where to write the completion file
    pub path: Option<PathBuf>,
}
