use super::GenerateContract;
use crate::types::{AbiEntry, ContractMetadata, IntermediateContracts, SolidityOutputFile};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, u64, u8};

#[derive(Deserialize, Debug)]
pub struct FoundryDeployData {
    pub transactions: Vec<FoundryTransaction>,
    pub chain: u64,
    pub multi: bool,
    pub commit: String,
}

// TODO: make fields private
#[derive(Deserialize, Debug)]
pub struct FoundryTransaction {
    #[serde(rename = "contractName")]
    pub contract_name: String,

    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

pub struct Foundry {
    contracts_path: PathBuf,
}

impl GenerateContract for Foundry {
    fn get_intermediate_contratcs(contracts_path: &PathBuf) -> IntermediateContracts {
        let output = std::process::Command::new("forge")
            .arg("build")
            .current_dir(contracts_path)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout).unwrap();

                    println!("{stdout}");

                    Foundry::get_intermediate_contratcs(contracts_path)
                } else {
                    panic!("Failed to compile contracts {:?}", output);
                }
            }
            Err(e) => {
                panic!("Failed to compile contracts: {e}");
            }
        }
    }
}

// TODO: add PathBuf to instance
impl Foundry {
    fn get_intermediate_contratcs(contracts_path: &PathBuf) -> IntermediateContracts {
        let chain_ids = Foundry::get_chain_ids(contracts_path);

        let mut intermiate_contracts: IntermediateContracts = HashMap::new();

        chain_ids
            .into_iter()
            .map(|chain_id| Foundry::get_deploy_data(contracts_path, chain_id))
            .map(|deploy_data| Foundry::parse_deploy_data(deploy_data, contracts_path))
            .for_each(|contracts| {
                intermiate_contracts.insert(contracts[0].chain, contracts);
            });

        intermiate_contracts
    }

    fn get_deploy_data(contracts_path: &PathBuf, chain_id: u64) -> FoundryDeployData {
        // get folders name inside folder
        let deploy_data_path = contracts_path
            .join("broadcast")
            .join("Deploy.s.sol")
            .join(chain_id.to_string())
            .join("run-latest.json");

        let raw_deploy_data =
            std::fs::read_to_string(deploy_data_path).expect("Failed to read deploy data");
        let deploy_data: FoundryDeployData =
            serde_json::from_str(&raw_deploy_data).expect("Failed to parse deploy data");

        deploy_data
    }

    fn parse_deploy_data(
        deploy_data: FoundryDeployData,
        contracts_path: &PathBuf,
    ) -> Vec<ContractMetadata> {
        deploy_data
            .transactions
            .into_iter()
            .filter(|transaction| {
                // Check if contract exists in the build folder
                // TODO: the contract can exist in a folder with another name, if this contratc is
                // defined inside a file that doesn't have the same name as the contract. But in
                // the meantime we're ignoring that
                contracts_path
                    .join("out")
                    .join(format!("{}.sol", transaction.contract_name))
                    .join(format!("{}.json", transaction.contract_name))
                    .exists()
            })
            .map(|transaction| {
                let abi = Foundry::get_abi(&transaction.contract_name, contracts_path);

                ContractMetadata {
                    address: transaction.contract_address,
                    name: transaction.contract_name,
                    abi,
                    bytecode: String::new(),
                    chain: deploy_data.chain,
                }
            })
            .collect::<Vec<ContractMetadata>>()
    }

    fn get_abi(contract_name: &String, contracts_path: &PathBuf) -> Vec<AbiEntry> {
        let abi_path = contracts_path
            .join("out")
            .join(format!("{contract_name}.sol"))
            .join(format!("{contract_name}.json"));

        let raw_abi = std::fs::read_to_string(abi_path).expect("Failed to read abi");
        let solidity_output: SolidityOutputFile =
            serde_json::from_str(&raw_abi).expect("Failed to parse abi");

        solidity_output.abi
    }

    fn get_chain_ids(contracts_path: &PathBuf) -> Vec<u64> {
        std::fs::read_dir(contracts_path.join("broadcast").join("Deploy.s.sol"))
            .unwrap()
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| {
                        e.path()
                            .file_name()
                            .map(|f| f.to_string_lossy().to_string())
                    })
                    .and_then(|filename| filename.parse::<u64>().ok())
            })
            .collect::<Vec<u64>>()
    }
}
