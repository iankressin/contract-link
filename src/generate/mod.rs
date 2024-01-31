pub mod contract_frameworks;
pub mod web3_frameworks;

use self::{
    contract_frameworks::CompileContracts,
    web3_frameworks::{viem::Viem, UpdateFiles},
};
use crate::{
    config_handler::ConfigHandler,
    types::{ContractsFramework, IntermediateContracts, Web3Framework},
};
use contract_frameworks::foundry::Foundry;
use std::path::PathBuf;

pub struct Generate;
impl Generate {
    pub fn generate(project_name: &String) {
        match ConfigHandler::get_config(project_name) {
            Some(config) => {
                let intermidate_contracts = Generate::compile_contracts(
                    config.contracts_framework,
                    config.contracts_dir
                );

                Generate::update_app(
                    config.web3_framework,
                    config.addresses_dir,
                    config.abi_dir,
                    intermidate_contracts,
                );

                println!("Files created successfully");
            }
            None => {
                println!("Config for project {project_name} not found");
                println!("Please run `contract-link config` to create a config file for this project.");
            }
        }
    }

    fn compile_contracts(
        contracts_framework: ContractsFramework,
        contracts_dir: PathBuf,
    ) -> IntermediateContracts {
        match contracts_framework {
            ContractsFramework::Foundry => Foundry::get_intermediate_contratcs(&contracts_dir),
            _ => panic!("Implement other frameworks"),
        }
    }

    // TODO: better error handling
    fn update_app(
        web3_frameworks: Web3Framework,
        addresses_dir: PathBuf,
        abi_dir: PathBuf,
        intermidiate_contracts: IntermediateContracts,
    ) {
        match web3_frameworks {
            Web3Framework::Viem => {
                Viem::new(abi_dir, addresses_dir, intermidiate_contracts)
                    .update_files()
                    .unwrap();
            }
            _ => {
                panic!("Implement other frameworks");
            }
        }
    }
}
