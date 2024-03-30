use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    base: String,
    map: HashMap<String, String>,
    alias: HashMap<String, String>,
}

impl Config {}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "base: {}", self.base)?;
        writeln!(f, "map:")?;
        for (key, value) in &self.map {
            writeln!(f, "  {}: {}", key, value)?;
        }
        writeln!(f, "Alias:")?;
        for (key, value) in &self.alias {
            writeln!(f, "  {}: {}", key, value)?;
        }

        anyhow::Result::Ok(())
    }
}

/// dux (Disk Utility X)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Available commands
#[derive(Debug, Subcommand)]
enum Command {
    /// Organizes files based on the preconfigured configmap
    Organize {
        #[clap(short = 'd')]
        dir: String,

        #[clap(short = 'c')]
        config: Option<String>,
    },

    /// Displays currently loaded config map
    Config,
    /// Reads config (testing purpose only)
    ReadConfig {
        #[clap(short = 'c')]
        config: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Note: should be something lile ~/.config/dux/dux.conf
    let default_config_path = "/Users/thearyanahmed/web/projects/dux/dux.json";

    match args.command {
        Command::Organize { dir, config: _ } => {
            println!("Run organize on {}", dir);

            // Steps:
            // Read the config,
            // Read aliases and put it inside the map
            // Read the base
            // Read from the dir
            // Put the files into a map, HashMap::<String<extension>, String<AbsolutePath>>
            // move the files to from that point to the target point
        }
        Command::Config => {
            println!("display config map");
        }
        Command::ReadConfig { config } => {
            let path = match config {
                Some(cf) => cf,
                None => default_config_path.to_string(),
            };

            println!("reading from {}", path);

            let config = read_config(path)?;
            println!("config {}", config);
        }
    }
    Ok(())
}

pub fn read_config(path: String) -> Result<Config> {
    // Note: need to add absolute path. The target is running from target/debug/dux. When it looks
    // for dux.json inside that, it would not find it.
    let dux_conf =
        fs::read_to_string(path).context(format!("failed to read config from given path"))?;

    // Parse the JSON string into a Config struct
    let config: Config = serde_json::from_str(&dux_conf).context("failed to map config file")?;

    Ok(config)
}
