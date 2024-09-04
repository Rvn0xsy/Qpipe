use std::path::Path;
use log::{error, info};
use crate::cli::Cli;
use serde::{Deserialize, Serialize};

const MODEL_NAME: &str = "glm-4-flash";
const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/chat/completions";

const PROMPT: &str = "You are an AI assistant. You will be given a task.";

const CONFIG_FILE: &str = "config.yaml";
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model: String,
    pub api_key: String,
    pub url: String,
    pub server: String,
    pub process_group: Vec<ProcessGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessGroup{
    pub name: String,
    pub cron: String,
    pub prompt: String,
    pub stream: String,
}

impl Default for Config {
    fn default() -> Self {
        let default_process_group = ProcessGroup {
            name: String::from("default"),
            cron: String::from("now"),
            prompt: String::from(PROMPT),
            stream: String::from("/path/to/script.py"),
        };

        Self {
            model: String::from(MODEL_NAME),
            api_key: String::from(""),
            url: String::from(API_URL),
            process_group: vec![default_process_group],
            server: String::from("127.0.0.1:6000"),
        }
    }
}

fn generate_default_config() -> Result<bool, &'static str> {
    let path = Path::new(CONFIG_FILE);
    match std::fs::metadata(path) {
        Ok(metadata) => {
            info!("config file exists: {:?}", metadata.is_file());
            Ok(true)
        }
        Err(e) => {
            // 文件不存在
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let  config = Config::default();
                    let yaml = serde_yaml::to_string(&config).unwrap();
                    std::fs::write("config.yaml", yaml).expect("error: failed to write config file");
                    info!("generate default config -> config.yaml");
                    Err("error: config file not provided. Please provide a config file path via `--config <path>")
                }
                _ => {
                    // 其他错误，比如权限问题
                    error!("Error checking file: {:?}", e);
                    Err("error: other error occurred")
                }
            }
        }
    }
}

impl Config {
    pub(crate) fn new(cli:&Cli) -> Result<Self, &'static str > {
        // 读取配置文件
        let mut config_path = CONFIG_FILE;
        if let Some(path) = cli.config.as_deref() {
            config_path = path.to_str().unwrap();
            eprintln!("error: config file not provided. Please provide a config file path via `--config <path>");
        }

        match std::fs::read_to_string(config_path) {
            Ok(yaml_str) => {
                Ok(serde_yaml::from_str(&yaml_str).unwrap())
            },
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        error!("error: config file not found at {}", config_path);
                        let  config = Config::default();
                        info!("generate default config -> config.yaml");
                        let yaml = serde_yaml::to_string(&config).unwrap();
                        std::fs::write("config.yaml", yaml).expect("error: failed to write config file");
                        Err("error: config file not found")
                    }
                    _ => {
                        error!("error: failed to read config file: {}", e);
                        Err("error: other error occurred")
                    }
                }
            }
        }
    }



    fn set_model_name(&mut self, model_name: &str) {
        self.model = model_name.to_string();
    }

    fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    fn get_config(&self) -> &Config {
        self
    }

    // 加载指定配置文件
    pub(crate) fn load_yaml_config(path: &str) -> Self {
        let yaml_str = std::fs::read_to_string(path).expect(&format!("failure read config file {}", path));
        serde_yaml::from_str(&yaml_str).unwrap()
    }
}

