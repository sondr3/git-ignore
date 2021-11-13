use crate::ignore::project_dirs;
use colored::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub path: PathBuf,
    pub aliases: HashMap<String, Vec<String>>,
    pub templates: HashMap<String, PathBuf>,
}

impl Config {
    pub fn create(force: bool) -> Result<(), Box<dyn std::error::Error>> {
        let dirs = project_dirs();

        let config_file: PathBuf = [
            dirs.config_dir()
                .to_str()
                .expect("Could not unwrap config directory"),
            "config.toml",
        ]
        .iter()
        .collect();

        Config::create_dir(dirs.config_dir());

        if config_file.exists() && !force {
            println!("{}: config already exist", "INFO".bold().blue());
            return Ok(());
        }

        if config_file.exists() && force {
            eprintln!("{}: overwriting existing config file", "WARN".bold().red());
        }

        let config = Config::new(config_file);
        config.write()
    }

    pub fn from_dir(path: &Path) -> Option<Self> {
        if path.exists() {
            let file = Path::new(&path);
            let file = read_to_string(file).unwrap();

            match toml::from_str::<Config>(&file).as_mut() {
                Ok(config) => {
                    config.path = path.to_path_buf();
                    Some(config.clone())
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn list_aliases(&self) {
        if self.aliases.is_empty() {
            return println!("{}", "No aliases defined".blue());
        }

        println!("{}", "Available aliases:".bold().green());
        for (name, aliases) in self.aliases.iter() {
            println!("{} => {:?}", name.blue(), aliases);
        }
    }

    pub fn add_alias(
        &mut self,
        name: String,
        aliases: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.aliases.insert(name, aliases);
        self.write()
    }

    pub fn remove_alias(&mut self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.aliases.remove(&name);
        self.write()
    }

    pub fn list_templates(&self) {
        if self.templates.is_empty() {
            return println!("{}", "No templates defined".blue());
        }

        println!("{}", "Available templates:".bold().green());
        for (name, path) in self.templates.iter() {
            println!("{} => {:?}", name.blue(), path);
        }
    }

    pub fn add_template(
        &mut self,
        name: String,
        path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.templates.insert(name, path);
        self.write()
    }

    pub fn remove_template(&mut self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.templates.remove(&name);
        self.write()
    }

    fn new(path: PathBuf) -> Self {
        Self {
            aliases: Default::default(),
            templates: Default::default(),
            path,
        }
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&self.path)?;
        file.write_all(toml::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }

    fn create_dir(path: &Path) {
        if !path.exists() {
            std::fs::create_dir_all(path).expect("Could not create config directory");
        }
    }
}
