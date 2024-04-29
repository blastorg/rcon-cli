use confy;
use inquire::Select;
use log::error;
use serde::{Deserialize, Serialize};
use std::{option::Option, process};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RconCliConfig {
    pub default_server: Option<ServerConfig>,
    pub servers: Vec<ServerConfig>,
}

impl RconCliConfig {
    pub fn new(selected_server: Option<ServerConfig>, servers: Vec<ServerConfig>) -> Self {
        RconCliConfig {
            default_server: selected_server,
            servers,
        }
    }

    pub fn add_server(&mut self, server: ServerConfig) {
        // validate that the server does not already exist
        for s in &self.servers {
            if s.name == server.name {
                panic!("Server name already exists in configuration");
            } else if s.ip == server.ip && s.port == server.port {
                error!(
                    "Server IP and port already exists in configuration under name: {}",
                    s.name
                );
                process::exit(2);
            }
        }
        self.servers.push(server);
    }

    pub fn clear(&mut self) {
        self.servers.clear();
        self.default_server = Option::None;
    }

    pub fn remove_server(&mut self, server: ServerConfig) {
        self.servers.retain(|s| s != &server);
        if self.default_server == Some(server) {
            self.default_server = Option::None
        }
    }

    fn has_server(&self, server: &ServerConfig) -> bool {
        for s in &self.servers {
            if s == server {
                return true;
            }
        }
        return false;
    }

    pub fn set_default_server(&mut self, server: ServerConfig) {
        if self.has_server(&server) {
            self.default_server = Some(server);
        } else {
            error!("Server does not exist in configuration");
        }
    }

    pub fn order_servers(&mut self) {
        self.servers
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub name: String,
    pub ip: String,
    pub port: String,
    pub pass: String,
}

impl ServerConfig {
    pub fn new(name: String, ip: String, port: String, pass: String) -> Self {
        ServerConfig {
            name,
            ip,
            port,
            pass,
        }
    }
}

/// `ServerConfig` implements `Display`
impl ::std::fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}: {}:{}; rcon_password {}",
            self.name, self.ip, self.port, self.pass
        )
    }
}

/// `RconCliConfig` implements `Default`
impl ::std::default::Default for RconCliConfig {
    fn default() -> Self {
        RconCliConfig {
            default_server: Option::None,
            servers: Vec::new(),
        }
    }
}

pub fn get_path() -> String {
    let path = confy::get_configuration_file_path("rcon_cli", None).unwrap();
    return path.to_str().unwrap().to_string();
}

pub fn get_config() -> RconCliConfig {
    let cfg: RconCliConfig = confy::load("rcon_cli", None).unwrap();
    return cfg;
}

pub fn save_config(cfg: RconCliConfig) {
    let mut config = cfg.clone();
    config.order_servers();
    confy::store("rcon_cli", None, config).unwrap();
}

pub fn set_and_save_default_server(server: ServerConfig) {
    let mut cfg: RconCliConfig = get_config();
    cfg.set_default_server(server);
    save_config(cfg);
}

pub fn select_server_from_list() -> ServerConfig {
    let cfg: RconCliConfig = get_config();
    let ans = Select::new("Selected server: ", cfg.servers.clone()).prompt();
    let selected_server = match ans {
        Ok(choice) => choice,
        Err(_) => panic!("There was an error, please try again"),
    };

    return selected_server;
}
