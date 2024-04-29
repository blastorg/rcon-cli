use anyhow::Error;
use clap::Parser;

mod args;
mod commands;
mod config;
mod shell_tools;

use args::{EntityType, RconActionsSubcommands, RconArgs, ServerManagementSubcommands};
use commands::{action, server};
use log::debug;
use shell_tools::completion;

fn main() -> Result<(), Error> {
    let args: RconArgs = RconArgs::parse();
    // Set logging level based on the verbosity flag passed by the user
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match args.entity_type {
        EntityType::Server(server) => match server.cmd {
            ServerManagementSubcommands::List => {
                server::list_servers();
                return Ok(());
            }

            ServerManagementSubcommands::Add => {
                server::add_server();
                return Ok(());
            }

            ServerManagementSubcommands::SetDefault => {
                server::set_default_server();
                return Ok(());
            }

            ServerManagementSubcommands::Edit => {
                server::edit_server();
                return Ok(());
            }

            ServerManagementSubcommands::Remove => {
                server::remove_server();
                return Ok(());
            }

            ServerManagementSubcommands::Clear => {
                server::clear_servers();
                return Ok(());
            }
        },
        EntityType::Action(action) => match action.cmd {
            RconActionsSubcommands::Shell => {
                debug!("Executing dynamic RCON command prompt");

                action::shell(args.default_server)?;
                return Ok(());
            }
            RconActionsSubcommands::Exec(cmd) => {
                debug!("Executing RCON command: {:?}", cmd);
                action::execute_command(args.default_server, cmd.command)?;
                return Ok(());
            }
        },
        EntityType::ShellCompletion(location) => completion(location.path),
    }
}
