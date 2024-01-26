pub mod foundry;

use std::path::PathBuf;
use crate::types::IntermidiateContract;

pub trait GenerateContract {
    fn get_intermediate_contratcs(contracts_path: &PathBuf) -> Vec<IntermidiateContract>;
}
