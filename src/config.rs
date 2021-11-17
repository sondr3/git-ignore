use crate::ignore::{project_dirs, Type};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

fn config_file() -> PathBuf {
    let dirs = project_dirs();

    [
        dirs.config_dir()
            .to_str()
            .expect("Could not unwrap config directory"),
        "config.toml",
    ]
    .iter()
    .collect()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub path: PathBuf,
    pub aliases: HashMap<String, Vec<String>>,
    pub templates: HashMap<String, String>,
}

impl Config {
    pub fn create(force: bool) -> Result<(), Box<dyn std::error::Error>> {
        let config_file = config_file();
        Config::create_dir(config_file.parent().unwrap());

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

    pub fn from_dir() -> Option<Self> {
        let config_file = config_file();
        if config_file.exists() {
            let file = Path::new(&config_file);
            let file = read_to_string(file).unwrap();

            match toml::from_str::<Config>(&file).as_mut() {
                Ok(config) => {
                    config.path = config_file;
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
        for (name, aliases) in &self.aliases {
            println!("{} => {:?}", name.blue(), aliases);
        }
    }

    pub fn add_alias(
        &mut self,
        name: String,
        aliases: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Created alias {} for {:?}", name.blue(), aliases);
        self.aliases.insert(name, aliases);
        self.write()
    }

    pub fn remove_alias(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.aliases.remove(name).is_some() {
            println!("Removed alias {}", name.blue());
        } else {
            println!("No alias named {} found", name.blue());
        }
        self.write()
    }

    pub fn list_templates(&self) {
        if self.templates.is_empty() {
            return println!("{}", "No templates defined".blue());
        }

        println!("{}", "Available templates:".bold().green());
        for (name, path) in &self.templates {
            println!("{} => {:?}", name.blue(), path);
        }
    }

    pub fn add_template(
        &mut self,
        name: String,
        file_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = self
            .path
            .parent()
            .unwrap()
            .join("templates")
            .join(&file_name);

        println!(
            "Created template {} at {}",
            name.blue(),
            file.to_str().unwrap_or_default().yellow()
        );

        let mut file = File::create(file)?;
        file.write_all(format!("\n### {} ###\n", name).as_bytes())?;

        self.templates.insert(name, file_name);
        self.write()
    }

    pub fn remove_template(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.templates.remove(name).is_some() {
            println!("Removed template {}", name.blue());
        } else {
            println!("No template named {} found", name.blue());
        }
        self.write()
    }

    pub fn names(&self) -> Vec<Type> {
        let aliases = self.aliases.keys();
        let templates = self.templates.keys();

        let mut res: Vec<_> = aliases.cloned().map(Type::Alias).collect();
        res.extend(templates.cloned().map(Type::Template));
        res.sort_unstable();

        res
    }

    pub fn read_template(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let dir = project_dirs().config_dir().join("templates").join(path);
        let content = read_to_string(dir)?;

        Ok(content)
    }

    fn new(path: PathBuf) -> Self {
        Self {
            aliases: HashMap::default(),
            templates: HashMap::default(),
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
            std::fs::create_dir_all(&path).expect("Could not create config directory");
        }

        let path = path.join("templates");
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Could not create config directory");
        }
    }
}
