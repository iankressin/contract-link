mod config_handler;
mod generate;
mod types;

use std::io::ErrorKind;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use inquire::{autocompletion::Replacement, Autocomplete, CustomUserError, Select, Text};
use types::{ContractsFramework, ProjectConfig, Web3Framework};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config,
    Generate {
        project: String,
    },
}

// TODO:
// - installation script
// - contracts build command
// - message saying that the files were created successfully
fn main() {
    match &Cli::parse().command {
        Some(Commands::Config) => {
            let project_name = Text::new("Pick a name for your project:").prompt().unwrap();
            let current_dir = std::env::current_dir().unwrap();
            let help_message = format!("Current directory: {}", current_dir.to_string_lossy());

            let contracts_dir_str: String = Text::new("Enter the path of your contracts project:")
                .with_autocomplete(FilePathCompleter::default())
                .with_help_message(&help_message)
                .prompt()
                .unwrap();
            let contracts_dir = PathBuf::from(contracts_dir_str);

            let contracts_framework = match Select::new("Select the framework used by your contracts:", vec!["Foundry", "Hardhat"]).prompt().unwrap() {
                "Foundry" => ContractsFramework::Foundry,
                "Hardhat" => ContractsFramework::Hardhat,
                _ => panic!("Invalid contract framework"),
            };

            let abi_dir_str = Text::new("Enter the path where you want to put the abi files:")
                .with_autocomplete(FilePathCompleter::default())
                .with_help_message(&help_message)
                .prompt()
                .unwrap();
            let abi_dir = PathBuf::from(abi_dir_str);

            let addresses_dir_str = Text::new("Enter the path where you want to put the addresses file:")
                .with_autocomplete(FilePathCompleter::default())
                .with_help_message(&help_message)
                .prompt()
                .unwrap();
            let addresses_dir = PathBuf::from(addresses_dir_str);

            let web3_framework = match Select::new("Select the framework used by app:", vec!["Viem", "Ethers"]).prompt().unwrap() {
                "Viem" => Web3Framework::Viem,
                "Ethers" => Web3Framework::Ethers,
                _ => panic!("Invalid web3 framework"),
            };

            let config = ProjectConfig {
                contracts_dir,
                contracts_framework,
                web3_framework,
                abi_dir,
                addresses_dir,
            };

            config_handler::ConfigHandler::create_config_file(&project_name, config);
        }
        Some(Commands::Generate { project }) => generate::Generate::generate(project),
        None => println!("No command provided"),
    }
}

#[derive(Clone, Default)]
pub struct FilePathCompleter {
    input: String,
    paths: Vec<String>,
    lcp: String,
}

impl FilePathCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }

        self.input = input.to_owned();
        self.paths.clear();

        let input_path = std::path::PathBuf::from(input);

        let fallback_parent = input_path
            .parent()
            .map(|p| {
                if p.to_string_lossy() == "" {
                    std::path::PathBuf::from(".")
                } else {
                    p.to_owned()
                }
            })
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let scan_dir = if input.ends_with('/') {
            input_path
        } else {
            fallback_parent.clone()
        };

        let entries = match std::fs::read_dir(scan_dir) {
            Ok(read_dir) => Ok(read_dir),
            Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_dir(fallback_parent),
            Err(err) => Err(err),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut idx = 0;
        let limit = 15;

        while idx < entries.len() && self.paths.len() < limit {
            let entry = entries.get(idx).unwrap();

            let path = entry.path();
            let path_str = if path.is_dir() {
                format!("{}/", path.to_string_lossy())
            } else {
                path.to_string_lossy().to_string()
            };

            if path_str.starts_with(&self.input) && path_str.len() != self.input.len() {
                self.paths.push(path_str);
            }

            idx = idx.saturating_add(1);
        }

        self.lcp = self.longest_common_prefix();

        Ok(())
    }

    fn longest_common_prefix(&self) -> String {
        let mut ret: String = String::new();

        let mut sorted = self.paths.clone();
        sorted.sort();
        if sorted.is_empty() {
            return ret;
        }

        let mut first_word = sorted.first().unwrap().chars();
        let mut last_word = sorted.last().unwrap().chars();

        loop {
            match (first_word.next(), last_word.next()) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    ret.push(c1);
                }
                _ => return ret,
            }
        }
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.paths.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => match self.lcp.is_empty() {
                true => Replacement::None,
                false => Replacement::Some(self.lcp.clone()),
            },
        })
    }
}
