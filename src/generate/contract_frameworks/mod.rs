pub mod foundry;

use crate::types::IntermediateContracts;
use std::path::PathBuf;

pub trait CompileContracts {
    fn get_intermediate_contratcs(contracts_path: &PathBuf) -> IntermediateContracts;
}
