use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::write;
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
    },

    /// Displays currently loaded config map
    Config,
    /// Reads config (testing purpose only)
    ReadConfig,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Organize { dir } => {
            println!("Run organize on {}", dir);
        }
        Command::Config => {
            println!("display config map");
        }

        Command::ReadConfig => {
            let config = read_config()?;
            println!("config {}", config);
        }
    }
    Ok(())
}

// TODO: config path should be a parameter
pub fn read_config() -> Result<Config> {
    // Note: need to add absolute path. The target is running from target/debug/dux. When it looks
    // for dux.json inside that, it would not find it.
    let dux_conf = fs::read_to_string("/Users/thearyanahmed/web/projects/dux/dux.json")
        .context("failed to read dux.json")?;

    // Parse the JSON string into a Config struct
    let config: Config = serde_json::from_str(&dux_conf).context("failed to map config file")?;

    Ok(config)
}
