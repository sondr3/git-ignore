use colored::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub aliases: HashMap<String, Vec<String>>,
    pub templates: HashMap<String, PathBuf>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl Config {
    pub fn create(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Config::create_dir(path);

        let config = Config::new(path.to_path_buf());
        config.write()
    }

    pub fn from_dir(path: &Path) -> Option<Self> {
        if path.exists() {
            let file = Path::new(&path);
            let file = read_to_string(file).unwrap();

            let result: Config = toml::from_str(&file).unwrap();
            Some(result)
        } else {
            None
        }
    }

    pub fn list_aliases(&self) {
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
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("Could not create config directory");
            }
        }
    }
}
