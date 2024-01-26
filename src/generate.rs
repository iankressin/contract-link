use std::path::PathBuf;

use crate::{config_handler::ConfigHandler, types::ContractsFramework};

pub struct Generate;

impl Generate {
    pub fn generate(project_name: &String) {
        match ConfigHandler::get_config(project_name) {
            Some(config) => {
                Generate::compile_contract(
                    config.contracts_framework,
                    config.contracts_path
                );
            }
            None => {
                println!("Config for project {project_name} not found");
                println!("Please run `contract-link config` to create a config file for this project.");
            }
        }
    }

    fn compile_contract(contracts_framework: ContractsFramework, contracts_path: PathBuf) {
        match contracts_framework {
            ContractsFramework::Foundry => Generate::generate_foundry(contracts_path),
            ContractsFramework::Hardhat => Generate::generate_hardhat(contracts_path),
            ContractsFramework::Truffle => Generate::generate_truffle(contracts_path),
        }
    }

    // compile
    // parse to web3 framework format
    // write to app directory
    fn generate_foundry(contracts_path: PathBuf) {
        println!("COMPILING ON: {:?}", contracts_path);

        let output = std::process::Command::new("forge")
            .arg("build")
            .current_dir(contracts_path)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    println!("{stdout}");
                } else {
                    println!("Failed to compile contracts");
                }
            }
            Err(e) => {
                println!("Failed to compile contracts: {e}");
            }
        }
    }

    fn generate_truffle(contracts_path: PathBuf) {
        let _output = std::process::Command::new("truffle")
            .arg("compile")
            .current_dir(contracts_path)
            .output()
            .expect("Failed to compile contracts");
    }

    fn generate_hardhat(contracts_path: PathBuf) {
        let _output = std::process::Command::new("hardhat")
            .arg("compile")
            .current_dir(contracts_path)
            .output()
            .expect("Failed to compile contracts");
    }
}
