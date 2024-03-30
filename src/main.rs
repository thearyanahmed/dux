use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::result::Result::Ok;

#[derive(Debug, Deserialize)]
pub struct Config {
    base: String,
    mapping: HashMap<String, String>,
    alias: HashMap<String, String>,
}

impl Config {}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "base: {}", self.base)?;
        writeln!(f, "mapping:")?;
        for (key, value) in &self.mapping {
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
    let default_config_path = String::from("/Users/thearyanahmed/web/projects/dux/dux.json");

    match args.command {
        Command::Organize { dir: _, config } => {
            let dir = "/Users/thearyanahmed/web/projects/dux/tests/fake/";

            println!("run organize on {}", dir);

            let config = parse_config(config, default_config_path)?;

            ensure_path_is_dir(dir)?;

            let dir = Path::new(dir);

            let files = list_files_recursive(dir)?;

            let mapped = map_files_by_extension(files);

            for (key, target_dir) in config.mapping {
                let parts: Vec<&str> = key.split(",").collect();

                for ext in parts {
                    if let Some(v) = mapped.get(ext) {
                        let base = config.base.clone();

                        let x = match base == "" {
                            true => {
                                let p = PathBuf::from(target_dir.clone());
                                p.to_string_lossy().to_string()
                            }
                            false => {
                                let mut path = PathBuf::from(base);
                                path.push(target_dir.clone());

                                path.to_string_lossy().to_string()
                            }
                        };
                        // Push the filename onto the PathBuf
                        println!("v {:?} \n x is {}", v, x);

                        println!("the final path is, {}", x);
                    }
                }
            }

            // loop over the extenions,
            // split them
            // inside the split, check if key exists in files vec,
            // if yes, move files to that location
            // else, display the problem
        }
        Command::Config => {
            println!("display config map");
        }
        Command::ReadConfig { config } => {
            let cfg = parse_config(config, default_config_path)?;

            println!("config -> {}", cfg)
        }
    }
    Ok(())
}

pub fn parse_config(config_path: Option<String>, default_config: String) -> Result<Config> {
    let path = match config_path {
        Some(cf) => cf.to_string(),
        None => default_config.to_string(),
    };

    let config = read_config(path)?;

    Ok(config)
}

pub fn map_files_by_extension(files: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in files {
        if let Some(extension) = file.extension().and_then(|e| e.to_str()) {
            map.entry(extension.to_string())
                .or_insert_with(Vec::new)
                .push(file);
        }
    }

    map
}

pub fn list_files_recursive(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mut subdir_files = list_files_recursive(&path)?;
            files.append(&mut subdir_files);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}

pub fn ensure_path_is_dir(path: &str) -> Result<()> {
    match fs::metadata(path) {
        core::result::Result::Ok(metadata) => match metadata.is_dir() {
            true => Ok(()),
            false => Err(anyhow!("{} is not a directory", path)),
        },
        Err(err) => Err(err.into()),
    }
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
