use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]pub struct GitHubConfig {
 pub token: String, pub username: String,
 pub default_private: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]pub struct NotesSyncConfig {
 pub repo: String,
 pub branch: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]pub struct Config {
 pub github: GitHubConfig,
 pub notes_sync: NotesSyncConfig,
}

impl Config {
    pub fn load() -> Config {
        let config_path = "flow_store/config.toml";
        if let Ok(config_content) = std::fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<Config>(&config_content) {
                return config;
            } else {
                println!("Ошибка при разборе конфигурационного файла. Используется конфигурация по умолчанию.");
            }
        } else {
            println!("Конфигурационный файл не найден. Используется конфигурация по умолчанию.");
        }
        Config::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = "flow_store/config.toml";
        if let Ok(config_content) = toml::to_string_pretty(self) {
            std::fs::write(config_path, config_content)?;
            Ok(())
        } else {
            Err("Ошибка при сериализации конфигурации.".into())
        }
    }
}

