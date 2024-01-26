pub mod contract_frameworks;
pub mod web3_frameworks;

use contract_frameworks::foundry::Foundry;
use std::path::PathBuf;
use crate::{config_handler::ConfigHandler, types::{ContractsFramework, IntermidiateContract, Web3Framework}};

use self::{contract_frameworks::GenerateContract, web3_frameworks::{viem::Viem, GenerateWeb3}};

pub struct Generate;
impl Generate {
    pub fn generate(project_name: &String) {
        match ConfigHandler::get_config(project_name) {
            Some(config) => {
                let intermidate_contracts = Generate::compile_contracts(
                    config.contracts_framework,
                    config.contracts_path
                );

                Generate::update_app(
                    config.web3_framework,
                    config.app_path,
                    intermidate_contracts
                );
            }
            None => {
                println!("Config for project {project_name} not found");
                println!("Please run `contract-link config` to create a config file for this project.");
            }
        }
    }

    fn compile_contracts(contracts_framework: ContractsFramework, contracts_path: PathBuf) -> Vec<IntermidiateContract> {
        match contracts_framework {
            ContractsFramework::Foundry => {
                Foundry::get_intermediate_contratcs(&contracts_path)
            },
            _ => {
                panic!("Implement other frameworks");
            }
        }
    }

    fn update_app(web3_frameworks: Web3Framework, contracts_path: PathBuf, intermidiate_contracts: Vec<IntermidiateContract>) {
        match web3_frameworks {
            Web3Framework::Viem => {
                // TODO: handle error
                Viem::generate(&contracts_path, intermidiate_contracts).unwrap();
            },
            _ => {
                panic!("Implement other frameworks");
            }
        }
    }
}
