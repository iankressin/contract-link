use std::{collections::HashMap, path::PathBuf};
use clap::ValueEnum;
use serde::{Serialize, Deserialize};

pub type Config = HashMap<String, ProjectConfig>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub contracts_path: PathBuf,
    pub app_path: PathBuf,
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
