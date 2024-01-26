use crate::types::IntermidiateContract;

use super::GenerateWeb3;
use std::{collections::HashMap, error, fs, path::PathBuf};

pub struct Viem;

impl GenerateWeb3 for Viem {
    fn generate(
        app_path: &PathBuf,
        intermidiate_contracts: Vec<IntermidiateContract>,
    ) -> Result<(), Box<dyn error::Error>> {
        Viem::generate_web3(app_path, &intermidiate_contracts)?;
        Viem::update_abi_files(app_path, &intermidiate_contracts)?;

        Ok(())
    }

    // create a contract address file
    // create/update abi files
    fn generate_web3(
        app_path: &PathBuf, 
        intermidiate_contracts: &Vec<IntermidiateContract>
    ) -> Result<(), Box<dyn error::Error>> {
        Viem::create_contract_addresses_file(app_path, intermidiate_contracts);

        Ok(())
    }

    fn update_abi_files(
        app_path: &PathBuf,
        intermidiate_contracts: &Vec<IntermidiateContract>
    ) -> Result<(), Box<dyn error::Error>> {
        intermidiate_contracts.iter().for_each(|contract| {
            let abi_file_path = app_path
                .join("src")
                .join("lib")
                .join("abis")
                .join(format!("{}.ts", contract.name));
            let abi_json = serde_json::to_string_pretty(&contract.abi).unwrap();

            println!("Serialized abi:{abi_json} {}", abi_file_path.display());

            let abi_ts = format!("export default {abi_json} as const;\n");
            fs::write(abi_file_path, abi_ts).expect("Failed to write abi file");
        });

        Ok(())
    }
}

impl Viem {
    fn create_contract_addresses_file(app_path: &PathBuf, intermediate_contracts: &Vec<IntermidiateContract>) {
        let mut contract_addresses_hashmap: HashMap<String, String> = std::collections::HashMap::new();

        intermediate_contracts.iter().for_each(|contract| {
            contract_addresses_hashmap.insert(
                contract.name.clone(),
                contract.address.clone()
            );
        });

        let contract_addresses_json = serde_json::to_string_pretty(&contract_addresses_hashmap).unwrap();
        let contract_address_ts = format!("export const ContractAddress = {contract_addresses_json} as const;\n");
        let contract_addresses_path = app_path.join("contract-addresses.ts");
        fs::write(contract_addresses_path, contract_address_ts).expect("Failed to write contract addresses file");
    }
}
