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
    .unwrap();

    // Auth request to RCON server (SERVERDATA_AUTH)
    client.auth(AuthRequest::new(server.pass)).unwrap();

    Ok(client)
}

fn determine_server(use_default_server: bool) -> ServerConfig {
    let cfg = config::get_config();
    if use_default_server {
        if cfg.default_server.is_none() {
            error!("No default server set, please add one with `rcon server set-default`",);
            std::process::exit(2);
        }
        println!(
            "▲ Default server is enabled: {}",
            cfg.default_server.as_ref().unwrap()
        );
        return cfg.default_server.unwrap();
    } else {
        return config::select_server_from_list();
    }
}

pub fn shell(use_default_server: bool) -> Result<(), RCONError> {
    let server: ServerConfig = determine_server(use_default_server);
    // Create new RCON client & validate auth
    let mut client = create_and_auth_client(server).unwrap();

    let mut continue_loop = true;
    while continue_loop {
        let command = Text::new("(q/quit) Enter RCON command:").prompt().unwrap();

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
    let mut client = create_and_auth_client(server).unwrap();

    let mut table = Table::new();

    let command_response = client.execute(RCONRequest::new(command))?;

    table.add_row(vec![Cell::new(command_response.body).fg(Color::Green)]);
    println!("▲\n{}", table);

    Ok(())
}
