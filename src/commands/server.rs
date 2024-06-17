use std::vec;

use crate::{config, config::RconCliConfig};
use comfy_table::{Attribute, Cell, Color, Table};
use inquire::{Confirm, Text};

use log::{debug, info};

pub fn add_server() {
    let name = Text::new("Name of server:")
        .prompt()
        .expect("Could not get server name");
    let ip: String = Text::new("Address:")
        .prompt()
        .expect("Could not get server address");
    let port: String = Text::new("Port:")
        .with_default("27015")
        .prompt()
        .expect("Could not get server port");
    let pass: String = Text::new("RCON password:")
        .prompt()
        .expect("Could not get RCON password");
    let set_as_default: bool = Confirm::new("Set as default server?")
        .prompt()
        .expect("Could not get default server setting");
    let correct_info = Confirm::new("Is the above information correct?")
        .prompt()
        .expect("Could not get confirmation");

    if !correct_info {
        info!("Server configuration not saved");
        return;
    }

    let server = config::ServerConfig::new(name, ip, port, pass);
    let mut new_config: RconCliConfig = config::get_config();
    new_config.add_server(server.clone());

    if set_as_default {
        new_config.set_default_server(server.clone());
    }

    info!("Adding server configuration to config: {}", server);
    config::save_config(new_config);
    list_servers();

    debug!("Configuration file saved at path: {:?}", config::get_path());
}

pub fn set_default_server() {
    let def_server = config::select_server_from_list();
    config::set_and_save_default_server(def_server);
    list_servers();
}

pub fn edit_server() {
    let server = config::select_server_from_list();
    let name = Text::new("Name of server:")
        .with_default(&server.name)
        .prompt()
        .expect("Could not get server name");
    let ip = Text::new("IP address:")
        .with_default(&server.ip)
        .prompt()
        .expect("Could not get server address");
    let port = Text::new("Port:")
        .with_default(&server.port)
        .prompt()
        .expect("Could not get server port");
    let pass = Text::new("RCON password:")
        .with_default(&server.pass)
        .prompt()
        .expect("Could not get RCON password");

    let override_settings = Confirm::new(&format!(
        "You want to override {} with the above info?",
        server.name
    ))
    .prompt()
    .expect("Could not get confirmation");

    if override_settings {
        let mut new_config: RconCliConfig = config::get_config();
        new_config.remove_server(server.clone());
        let updated_server = config::ServerConfig::new(name, ip, port, pass);
        new_config.add_server(updated_server.clone());
        config::save_config(new_config);
        info!("Updated server configuration: {}", updated_server);
        list_servers();
    }
}

pub fn clear_servers() {
    let confirm_prompt = Confirm::new("Are you sure you want to clear all server configurations?")
        .prompt()
        .expect("Could not get confirmation");
    if confirm_prompt {
        let mut cfg: RconCliConfig = config::get_config();
        cfg.clear();
        config::save_config(cfg);
        info!("Cleared all server configurations");

        list_servers();
    }
}

pub fn remove_server() {
    let cfg: RconCliConfig = config::get_config();
    let server = config::select_server_from_list();
    let confirm_prompt = Confirm::new(&format!(
        "Are you sure you want to remove server named: {}?",
        server.name
    ))
    .prompt()
    .expect("Could not get confirmation");
    if confirm_prompt {
        let mut new_config = RconCliConfig::new(cfg.default_server, cfg.servers);
        new_config.remove_server(server.clone());
        config::save_config(new_config);
        info!("Removed server configuration: {}", server.name);

        list_servers();
    }
}

pub fn list_servers() {
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Name").add_attribute(Attribute::Bold),
        Cell::new("Address").add_attribute(Attribute::Bold),
        Cell::new("Port").add_attribute(Attribute::Bold),
        Cell::new("RCON Password").add_attribute(Attribute::Bold),
        Cell::new("Default").add_attribute(Attribute::Bold),
    ]);

    let cfg: RconCliConfig = config::get_config();
    for server in &cfg.servers {
        if let Some(ref default) = cfg.default_server {
            if server == default {
                table.add_row(vec![
                    Cell::new(&server.name).fg(Color::Green),
                    Cell::new(&server.ip).fg(Color::Green),
                    Cell::new(&server.port).fg(Color::Green),
                    Cell::new(&server.pass).fg(Color::Green),
                    Cell::new("✓").fg(Color::Green),
                ]);
                continue;
            }
        }

        table.add_row(vec![
            Cell::new(&server.name),
            Cell::new(&server.ip),
            Cell::new(&server.port),
            Cell::new(&server.pass),
            Cell::new(""),
        ]);
    }
    println!("▲\n{}", table);
}
