pub mod viem;

use std::{error, path::PathBuf};

use crate::types::IntermidiateContract;

pub trait GenerateWeb3 {
    fn generate(
        app_path: &PathBuf,
        intermidiate_contracts: Vec<IntermidiateContract>,
    ) -> Result<(), Box<dyn error::Error>>;

    fn generate_web3(
        app_path: &PathBuf,
        intermidiate_contracts: &Vec<IntermidiateContract>,
     ) -> Result<(), Box<dyn error::Error>>;

    fn update_abi_files(
        app_path: &PathBuf,
        intermidiate_contracts: &Vec<IntermidiateContract>,
    ) -> Result<(), Box<dyn error::Error>>;
}
