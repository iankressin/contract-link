use crate::types::{Config, ProjectConfig};
use std::{collections::HashMap, env, fs};

pub struct ConfigHandler;

impl ConfigHandler {
    pub fn create_config_file(project_name: &String, project_config: ProjectConfig) {
        let home_dir = ConfigHandler::get_home_dir();
        let config_file_path = ConfigHandler::get_config_file_path();

        fs::create_dir_all(format!("{home_dir}/.contract-link")).unwrap();

        let config_str = fs::read_to_string(&config_file_path).unwrap_or(String::from("{}"));
        let mut config = serde_json::from_str::<Config>(&config_str).unwrap_or(HashMap::new());

        config.insert(project_name.to_string(), project_config);

        fs::write(
            &config_file_path,
            serde_json::to_string_pretty(&config).unwrap(),
        )
        .unwrap();

        println!("Config file created at: {config_file_path}",);
    }

    pub fn get_config(project_name: &String) -> Option<ProjectConfig> {
        let config_file_path = ConfigHandler::get_config_file_path();
        let current_config = fs::read_to_string(&config_file_path).unwrap_or(String::from("{}"));
        let config = serde_json::from_str::<Config>(&current_config).unwrap_or(HashMap::new());

        config
            .get(project_name)
            .map(|project_config| project_config.to_owned())
    }

    fn get_config_file_path() -> String {
        let home_dir = ConfigHandler::get_home_dir();
        format!("{home_dir}/.contract-link/config")
    }

    fn get_home_dir() -> String {
        let home_dir = if cfg!(windows) {
            env::var("USERPROFILE")
        } else {
            env::var("HOME")
        };

        match home_dir {
            Ok(path) => path,
            Err(e) => panic!("Could not find home directory: {e}"),
        }
    }
}
