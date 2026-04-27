mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "vaultls",
    version,
    about = "List and diff secrets across HashiCorp Vault namespaces"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List secrets at a given Vault path
    List {
        /// KV path to list, e.g. `secret/data/myapp`
        path: String,
    },
    /// Diff secrets between two Vault paths
    Diff {
        /// First KV path
        path_a: String,
        /// Second KV path
        path_b: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::Config::from_env()?;

    match cli.command {
        Commands::List { path } => {
            println!(
                "{} {}/{}",
                "Listing secrets at".cyan().bold(),
                cfg.vault_addr.yellow(),
                path.green()
            );
            // TODO: implement Vault KV list via vault::Client
        }
        Commands::Diff { path_a, path_b } => {
            println!(
                "{} {} {} {}",
                "Diffing".cyan().bold(),
                path_a.green(),
                "vs".cyan(),
                path_b.yellow()
            );
            // TODO: implement Vault KV diff
        }
    }

    Ok(())
}
