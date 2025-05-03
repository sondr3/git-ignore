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

#[derive(Deserialize, Serialize, Debug)]
pub struct Language {
    key: String,
    name: String,
    #[serde(rename = "fileName")]
    file_name: String,
    pub contents: String,
}

#[derive(Debug)]
pub struct IgnoreData {
    pub data: Vec<Type>,
}

impl IgnoreData {
    pub fn new(path: &Path, config: &Config) -> Result<Self> {
        let file = read_to_string(path)?;
        let templates: HashMap<String, Language> = serde_json::from_str(&file)?;

        let mut data: Vec<_> = templates
            .into_values()
            .map(|v| Type::Template {
                key: v.key,
                content: v.contents,
            })
            .collect();

        data.extend(
            config
                .aliases
                .clone()
                .into_iter()
                .map(|(k, v)| Type::Alias { key: k, aliases: v }),
        );

        let user_templates: Vec<_> = config
            .templates
            .clone()
            .into_iter()
            .map(|(name, path)| {
                let template = Config::read_template(&path)?;
                Ok(Type::UserTemplate {
                    key: name,
                    content: template,
                })
            })
            .collect::<Result<_>>()?;
        data.extend(user_templates);

        data.sort_unstable();

        Ok(IgnoreData { data })
    }

    pub fn get_template(&self, name: &str) -> Option<String> {
        self.data
            .iter()
            .find(|k| matches!(k,Type::Template { key, .. } if key == name))
            .map(|v| match v {
                Type::Template { content, .. } => content.clone(),
                _ => unreachable!(),
            })
    }

    pub fn get_alias(&self, name: &str) -> Option<Vec<String>> {
        self.data
            .iter()
            .find(|k| matches!(k,Type::Alias { key, .. } if key == name))
            .map(|v| match v {
                Type::Alias { aliases, .. } => aliases.clone(),
                _ => unreachable!(),
            })
    }

    pub fn get_user_template(&self, name: &str) -> Option<String> {
        self.data
            .iter()
            .find(|k| matches!(k,Type::UserTemplate { key, .. } if key == name))
            .map(|v| match v {
                Type::UserTemplate { content, .. } => content.clone(),
                _ => unreachable!(),
            })
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Template { key: String, content: String },
    Alias { key: String, aliases: Vec<String> },
    UserTemplate { key: String, content: String },
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Template { key: k1, .. }, Type::Template { key: k2, .. }) => k1 == k2,
            (Type::Alias { key: k1, .. }, Type::Alias { key: k2, .. }) => k1 == k2,
            (Type::UserTemplate { key: k1, .. }, Type::UserTemplate { key: k2, .. }) => k1 == k2,
            _ => false,
        }
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
        match (self, other) {
            (Type::UserTemplate { .. }, Type::UserTemplate { .. }) => Ordering::Equal,
            (Type::UserTemplate { .. }, _) => Ordering::Greater,
            (Type::Alias { .. }, Type::UserTemplate { .. }) => Ordering::Greater,
            (Type::Alias { .. }, Type::Alias { .. }) => Ordering::Equal,
            (Type::Alias { .. }, Type::Template { .. }) => Ordering::Less,
            (Type::Template { .. }, Type::Template { .. }) => Ordering::Equal,
            (Type::Template { .. }, _) => Ordering::Less,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeName {
    Template(String),
    Alias(String),
    UserTemplate(String),
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::Template(name) => write(f, format_args!("{}", name)),
            TypeName::Alias(name) => write(f, format_args!("{}", name.yellow())),
            TypeName::UserTemplate(name) => write(f, format_args!("{}", name.blue())),
        }
    }
}

impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl Eq for TypeName {}

impl PartialOrd for TypeName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypeName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner().cmp(other.inner())
    }
}

impl Hash for TypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state);
    }
}

impl TypeName {
    fn inner(&self) -> &str {
        match self {
            TypeName::Template(name) | TypeName::Alias(name) | TypeName::UserTemplate(name) => name,
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        let inner = self.inner();
        inner.contains(name)
    }
}
