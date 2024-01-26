use std::{collections::HashMap, env, fs, path::PathBuf};
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Create the config file under /home/usr/.contract-link/config
    Config {
        #[arg(short, long)]
        project: String,

        #[arg(long, value_name = "CONTRACTS DIRECTORY")]
        contracts_path: PathBuf,

        #[arg(short, long, value_name = "APP DIRECTORY")]
        app_path: PathBuf,

        // TODO: predefined strings
        #[arg(long)]
        contracts_framework: String,

        // TODO: predefined strings
        #[arg(short, long)]
        web3_framework: String,
    },

    Generate {
        project: String,
    } 
}

type Config = HashMap<String, ProjectConfig>;

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    contracts_path: PathBuf,
    app_path: PathBuf,
    contracts_framework: String,
    web3_framework: String,
}

struct ConfigHandler;

impl ConfigHandler {
    fn create_config_file(project_name: &String, project_config: ProjectConfig) {
        let home_dir = ConfigHandler::get_home_dir();
        let config_file_path = ConfigHandler::get_config_file_path();

        fs::create_dir_all(format!("{home_dir}/.contract-link")).unwrap();

        let current_config = fs::read_to_string(&config_file_path).unwrap_or(String::from("{}"));

        let mut config = serde_json::from_str::<Config>(&current_config).unwrap_or(HashMap::new());

        config.insert(project_name.to_string(), project_config);

        let config = serde_json::to_string_pretty(&config).unwrap();

        fs::write(&config_file_path, config).unwrap();

        println!("Config file created at: {config_file_path}",);
    }
     
    fn get_home_dir() ->  String {
        let home_dir = if cfg!(windows) {
            env::var("USERPROFILE")
        } else {
            env::var("HOME")
        };

        match home_dir {
            Ok(path) => path,
            Err(e) => panic!("Could not find home directory: {e}")
        }
    }

    fn get_config_file(project_name: &String) -> Option<ProjectConfig> {
        let config_file_path = ConfigHandler::get_config_file_path();
        let current_config = fs::read_to_string(&config_file_path).unwrap_or(String::from("{}"));
        let mut config = serde_json::from_str::<Config>(&current_config).unwrap_or(HashMap::new());
        let mut config = serde_json::from_str::<Config>(&current_config).unwrap_or(HashMap::new());

        config.get(project_name).map(|project_config| project_config.clone_into(target))
    }

    fn get_config_file_path() -> String {
        let home_dir = ConfigHandler::get_home_dir();
        format!("{home_dir}/.contract-link/config")
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config { project, contracts_framework, contracts_path, app_path, web3_framework }) => {
            let config = ProjectConfig {
                contracts_path: contracts_path.to_path_buf(),
                app_path: app_path.to_path_buf(),
                contracts_framework: contracts_framework.to_string(),
                web3_framework: web3_framework.to_string(),
            };

            ConfigHandler::create_config_file(project, config);


        }
        Some(Commands::Generate { project }) => {
        }
        None => {}
    }
}
