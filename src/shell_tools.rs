use std::{fs::File, path::PathBuf};

use anyhow::Error;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use log::error;

use crate::args::RconArgs;

pub fn completion(path: Option<PathBuf>) -> Result<(), Error> {
    if let Some(shell) = Shell::from_env() {
        let mut cli = RconArgs::command();
        let name = cli.get_name().to_string();

        if let Some(location) = path {
            generate(shell, &mut cli, name, &mut File::create(&location)?);
            println!("Shell autocomplete script generated at: {:?}", location)
        } else {
            error!("▲ Could not determine location to generate shell completion script");
            match &shell {
                Shell::PowerShell => {
                    eprintln!("▲ example usage: \n\t$ rcon completions rcon_completion.ps1");
                    eprintln!("\t$ .\\rcon_completion.ps1");
                }
                _ => {
                    eprintln!("▲ example usage: \n\t$ rcon completions rcon_completion.sh");
                    eprintln!("\t$ ./rcon_completion.sh");
                }
            }
        }
    } else {
        eprintln!("▲ Shell not supported");
    }
    Ok(())
}
