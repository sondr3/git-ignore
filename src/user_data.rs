use std::{
    collections::HashMap,
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::{Context, Result};
use colored::Colorize;
use etcetera::AppStrategy;
use serde::{Deserialize, Serialize};

use crate::ignore::PROJECT_DIRS;

static CONFIG_FILE: LazyLock<PathBuf> =
    LazyLock::new(|| PROJECT_DIRS.config_dir().join("config.toml"));

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct UserData {
    pub aliases: HashMap<String, Vec<String>>,
    pub templates: HashMap<String, String>,
}

impl UserData {
    pub fn create(force: bool) -> Result<()> {
        UserData::create_dir(
            CONFIG_FILE
                .parent()
                .context("No parent dir for the config_file")?,
        );

        if CONFIG_FILE.exists() && !force {
            eprintln!("{}: config already exist", "INFO".bold().blue());
            return Ok(());
        }

        if CONFIG_FILE.exists() && force {
            eprintln!("{}: overwriting existing config file", "WARN".bold().red());
        }

        let config = UserData::default();
        config.write()
    }

    pub fn new() -> Result<Self> {
        if CONFIG_FILE.exists() {
            match read_to_string(CONFIG_FILE.as_path()) {
                Ok(content) => {
                    toml::from_str::<UserData>(&content).context("could not parse config")
                }
                Err(_) => anyhow::bail!("could not read config file"),
            }
        } else {
            Ok(UserData::default())
        }
    }

    pub fn add_alias(&mut self, name: String, aliases: Vec<String>) -> Result<()> {
        println!("Created alias {} for {:?}", name.blue(), aliases);
        self.aliases.insert(name, aliases);
        self.write()
    }

    pub fn remove_alias(&mut self, name: &str) -> Result<()> {
        if self.aliases.remove(name).is_some() {
            println!("Removed alias {}", name.blue());
        } else {
            println!("No alias named {} found", name.blue());
        }
        self.write()
    }

    pub fn add_template(&mut self, name: String, file_name: String) -> Result<()> {
        let file = PROJECT_DIRS
            .config_dir()
            .parent()
            .context("Could not get parent directory of config file")?
            .join("templates")
            .join(&file_name);

        println!(
            "Created template {} at {}",
            name.blue(),
            file.to_str().unwrap_or_default().yellow()
        );

        let mut file = File::create(file)?;
        file.write_all(format!("### {} ###\n", name).as_bytes())?;

        self.templates.insert(name, file_name);
        self.write()
    }

    pub fn remove_template(&mut self, name: &str) -> Result<()> {
        if self.templates.remove(name).is_some() {
            println!("Removed template {}", name.blue());
        } else {
            println!("No template named {} found", name.blue());
        }
        self.write()
    }

    pub fn read_template(path: &str) -> Result<String> {
        let dir = PROJECT_DIRS.config_dir().join("templates").join(path);
        let content = read_to_string(dir)?;

        Ok(content)
    }

    fn write(&self) -> Result<()> {
        let mut file = File::create(CONFIG_FILE.as_path())?;
        file.write_all(toml::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }

    fn create_dir(path: &Path) {
        if !path.exists() {
            std::fs::create_dir_all(path).expect("Could not create config directory");
        }

        let path = path.join("templates");
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Could not create templates directory");
        }
    }
}
