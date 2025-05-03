use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, write},
    fs::read_to_string,
    hash::{Hash, Hasher},
    path::Path,
};

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug)]
pub struct IgnoreData {
    pub templates: HashMap<String, Language>,
    pub aliases: HashMap<String, Vec<String>>,
    pub user_templates: HashMap<String, String>,
}

impl IgnoreData {
    pub fn new(path: &Path, config: &Config) -> Result<Self> {
        let file = Path::new(path);
        let file = read_to_string(file)?;

        let templates: HashMap<String, Language> = serde_json::from_str(&file)?;

        let aliases = config.aliases.clone();
        let user_templates = config
            .templates
            .clone()
            .into_iter()
            .map(|(name, path)| {
                let template = Config::read_template(&path)?;
                Ok((name, template))
            })
            .collect::<Result<_>>()?;

        Ok(IgnoreData {
            templates,
            aliases,
            user_templates,
        })
    }

    pub fn get_template(&self, name: &str) -> Option<&str> {
        if let Some(res) = self.user_templates.get(name) {
            Some(res)
        } else if let Some(res) = self.templates.get(name) {
            Some(&res.contents)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Language {
    key: String,
    name: String,
    #[serde(rename = "fileName")]
    file_name: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
pub enum Type {
    Normal(String),
    Alias(String),
    Template(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Normal(name) => write(f, format_args!("{}", name)),
            Type::Alias(name) => write(f, format_args!("{}", name.yellow())),
            Type::Template(name) => write(f, format_args!("{}", name.blue())),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl Eq for Type {}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner().cmp(other.inner())
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state);
    }
}

impl Type {
    fn inner(&self) -> &str {
        match self {
            Type::Normal(name) | Type::Alias(name) | Type::Template(name) => name,
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        let inner = self.inner();
        inner.contains(name)
    }
}
