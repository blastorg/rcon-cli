use comfy_table::{Cell, Color, Table};
use log::error;
use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};

use crate::config::{self, ServerConfig};
use inquire::Text;

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
    let mut client = create_and_auth_client(server).expect("Could not create RCON client");

    let mut continue_loop = true;
    while continue_loop {
        let command = Text::new("(q/quit) Enter RCON command:")
            .prompt()
            .expect("Could not get input");

        if command == "q" || command == "quit" {
            continue_loop = false;
            continue;
        }

        let mut table = Table::new();

        let command_response = client.execute(RCONRequest::new(command))?;

        table.add_row(vec![Cell::new(command_response.body).fg(Color::Green)]);
        println!("▲\n{}", table);
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
