use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, u64};

pub type Config = HashMap<String, ProjectConfig>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub contracts_dir: PathBuf,
    pub abi_dir: PathBuf,
    pub addresses_dir: PathBuf,
    pub contracts_framework: ContractsFramework,
    pub web3_framework: Web3Framework,
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum ContractsFramework {
    Truffle,
    Hardhat,
    Foundry,
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum Web3Framework {
    Ethers,
    Web3,
    Viem,
}

pub type IntermediateContracts = HashMap<u64, Vec<ContractMetadata>>;

#[derive(Debug, Clone)]
pub struct ContractMetadata {
    pub address: String,
    pub name: String,
    pub abi: Vec<AbiEntry>,
    pub bytecode: String,
    pub chain: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SolidityOutputFile {
    pub abi: Vec<AbiEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiEntry {
    #[serde(rename = "type")]
    pub function_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub inputs: Vec<AbiInput>,

    #[serde(default)]
    pub outputs: Option<Vec<AbiOutput>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable: Option<bool>,

    #[serde(rename = "stateMutability", skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiInput {
    pub name: String,

    #[serde(rename = "type")]
    pub input_type: String,

    #[serde(rename = "internalType", skip_serializing_if = "Option::is_none")]
    pub internal_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiOutput {
    pub name: String,

    #[serde(rename = "type")]
    pub output_type: String,

    #[serde(rename = "internalType", skip_serializing_if = "Option::is_none")]
    pub internal_type: Option<String>,
}
