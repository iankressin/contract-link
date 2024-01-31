pub mod viem;

use std::error;

use crate::types::ContractMetadata;

pub trait UpdateFiles {
    fn update_files(&self) -> Result<(), Box<dyn error::Error>>;

    fn update_addresses_file(&self) -> Result<(), Box<dyn error::Error>>;

    fn update_abi_files(
        &self, 
        contract_metadate: &Vec<ContractMetadata>
    ) -> Result<(), Box<dyn error::Error>>;
}
