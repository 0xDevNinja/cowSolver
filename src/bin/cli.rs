use clap::{Parser, Subcommand};
use anyhow::Result;

/// Command-line interface for cowSolver.
#[derive(Debug, Parser)]
#[command(name = "cowSolver", version, about = "CoW Protocol cross-chain solver CLI")]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the solver CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the solver engine daemon service.
    Run {
        /// Path to configuration file.
        #[arg(short, long, default_value_t = String::from("config.toml"))]
        config: String,
    },
    /// Submit a single order directly via CLI.
    Submit {
        /// Token address to sell (e.g. ERC20 address).
        sell_token: String,
        /// Token address to buy.
        buy_token: String,
        /// Amount of sell token to sell.
        sell_amount: f64,
        /// Minimum amount of buy token to receive.
        buy_amount: f64,
        /// Target chain on which to place the order (e.g. "EthereumMainnet").
        #[arg(short, long, default_value_t = String::from("EthereumMainnet"))]
        chain: String,
    },
    /// Execute integration tests from the command line.
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { config } => {
            println!("Starting solver daemon with config {}...", config);
            // TODO: Call daemon service start
        }
        Commands::Submit {
            sell_token,
            buy_token,
            sell_amount,
            buy_amount,
            chain,
        } => {
            println!(
                "Submitting order: sell {} {} for {} {} on chain {}",
                sell_amount, sell_token, buy_amount, buy_token, chain
            );
            // TODO: Create Order struct and submit to solver service
        }
        Commands::Test => {
            println!("Running integration tests...");
            // TODO: trigger integration test suite
        }
    }
    Ok(())
}
