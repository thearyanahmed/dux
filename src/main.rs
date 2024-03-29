use anyhow::Ok;
use clap::{Parser, Subcommand};

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
    }
    Ok(())
}
