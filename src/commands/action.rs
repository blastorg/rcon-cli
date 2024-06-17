use comfy_table::{Cell, Color, Table};
use log::error;
use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::config::{self, ServerConfig};

fn create_and_auth_client(server: ServerConfig) -> Result<RCONClient, RCONError> {
    let mut client = RCONClient::new(RCONConfig {
        url: format!("{}:{}", server.ip, server.port),
        // Optional
        read_timeout: Some(13),
        write_timeout: Some(37),
    })
    .expect("Could not create RCON client");

    // Auth request to RCON server using stored password
    client
        .auth(AuthRequest::new(server.pass))
        .expect("Could not authenticate with RCON server");

    Ok(client)
}

fn determine_server(use_default_server: bool) -> ServerConfig {
    let cfg = config::get_config();
    if use_default_server {
        if cfg.default_server.is_none() {
            error!("No default server set, please add one with `rcon server set-default`",);
            std::process::exit(2);
        }
        let default_server = cfg.default_server.expect("No default server set");
        println!("▲ Default server is enabled: {}", default_server);
        return default_server;
    } else {
        return config::select_server_from_list();
    }
}

pub fn shell(use_default_server: bool) -> Result<(), RCONError> {
    let server: ServerConfig = determine_server(use_default_server);
    // Create new RCON client & validate auth
    let mut client = create_and_auth_client(&server).expect("Could not create RCON client");
    let mut rl = DefaultEditor::new().expect("Could not create shell editor");

    loop {
        println!(
            "▲ (q/quit) Shell mode for server: {}: {}",
            server.name, server.ip
        );
        let readline = rl.readline("▶︎ ");

        match readline {
            Ok(command) => {
                if command == "q" || command == "quit" {
                    println!("▶︎ Exiting, goodbye!");
                    break;
                }

                if let Err(err) = rl.add_history_entry(command.as_str()) {
                    eprintln!("Failed to add history entry: {}", err);
                }

                let mut table = Table::new();

                let command_response = client.execute(RCONRequest::new(command))?;

                table.add_row(vec![Cell::new(command_response.body).fg(Color::Green)]);
                println!("▲\n{}", table);
            }

            Err(ReadlineError::Interrupted) => {
                // CTRL-C"
                println!("▶︎ Exiting, goodbye!");
                break;
            }

            Err(ReadlineError::Eof) => {
                // CTRL-D
                println!("▶︎ Exiting, goodbye!");
                break;
            }

            Err(err) => {
                println!("▼ Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

pub fn execute_command(use_default_server: bool, command: String) -> Result<(), RCONError> {
    let server: ServerConfig = determine_server(use_default_server);
    // Create new RCON client & validate auth
    let mut client = create_and_auth_client(server).expect("Could not create RCON client");

    let mut table = Table::new();

    let command_response = client.execute(RCONRequest::new(command))?;

    table.add_row(vec![Cell::new(command_response.body).fg(Color::Green)]);
    println!("▲\n{}", table);

    Ok(())
}
