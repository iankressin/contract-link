mod generate;
mod config_handler;
mod types;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use types::{ProjectConfig, Web3Framework, ContractsFramework};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[arg(short, long)]
        project: String,

        #[arg(long, value_name = "CONTRACTS DIRECTORY")]
        contracts_path: PathBuf,

        #[arg(long, value_name = "APP DIRECTORY")]
        app_path: PathBuf,

        #[arg(long, value_name = "CONTRACTS FRAMEWORK")]
        contracts_framework: ContractsFramework,

        #[arg(short, long, value_name = "WEB3 FRAMEWORK")]
        web3_framework: Web3Framework,
    },

    Generate {
        project: String,
    } 
}

fn main() {
    match &Cli::parse().command {
        Some(Commands::Config { project, contracts_framework, contracts_path, app_path, web3_framework }) => {
            let config = ProjectConfig {
                contracts_path: contracts_path.to_path_buf(),
                app_path: app_path.to_path_buf(),
                contracts_framework: contracts_framework.clone(),
                web3_framework: web3_framework.clone(),
            };

            config_handler::ConfigHandler::create_config_file(project, config);
        }
        Some(Commands::Generate { project }) => generate::Generate::generate(project),
        None => println!("No command provided"),
    }
}
