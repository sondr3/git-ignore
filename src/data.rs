use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, write},
    fs::read_to_string,
    hash::{Hash, Hasher},
    path::PathBuf,
    sync::LazyLock,
};

use anyhow::Result;
use colored::Colorize;
use etcetera::AppStrategy;
use serde::{Deserialize, Serialize};

use crate::{ignore::PROJECT_DIRS, user_data::UserData};

pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| PROJECT_DIRS.cache_dir());
pub static CACHE_FILE: LazyLock<PathBuf> =
    LazyLock::new(|| PROJECT_DIRS.cache_dir().join("ignore.json"));

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
    pub fn new(user_data: &UserData) -> Result<Self> {
        let file = read_to_string(CACHE_FILE.as_path())?;
        let templates: HashMap<String, Language> = serde_json::from_str(&file)?;

        let mut data: Vec<_> = templates
            .into_values()
            .map(|v| Type::Template {
                key: v.key,
                content: v.contents,
            })
            .collect();

        data.extend(
            user_data
                .aliases
                .clone()
                .into_iter()
                .map(|(k, v)| Type::Alias { key: k, aliases: v }),
        );

        let user_templates: Vec<_> = user_data
            .templates
            .clone()
            .into_iter()
            .map(|(name, path)| {
                let template = UserData::read_template(&path)?;
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

    pub fn keys(&self) -> impl Iterator<Item = TypeName> {
        self.data.iter().map(TypeName::from)
    }

    pub fn list_aliases(&self) {
        let aliases = self
            .data
            .iter()
            .filter(|v| matches!(v, Type::Alias { .. }))
            .collect::<Vec<_>>();

        if aliases.is_empty() {
            return println!("{}", "No aliases defined".blue());
        }

        println!("{}", "Available aliases:".bold().green());
        for kind in aliases {
            println!(
                "{} => {:?}",
                TypeName::from(kind),
                self.get_alias(kind.key())
                    .expect("Found alias is missing, this is an internal error")
            );
        }
    }

    pub fn list_templates(&self) {
        let templates = self
            .data
            .iter()
            .filter(|v| matches!(v, Type::UserTemplate { .. }))
            .collect::<Vec<_>>();

        if templates.is_empty() {
            return println!("{}", "No templates defined".blue());
        }

        println!("{}", "Available templates:".bold().green());
        for kind in templates {
            println!(
                "{}:\n{}",
                TypeName::from(kind),
                self.get_user_template(kind.key())
                    .expect("Found template is missing, this is an internal error")
            );
        }
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

impl Type {
    pub fn key(&self) -> &str {
        match self {
            Self::Template { key, .. } => key,
            Self::Alias { key, .. } => key,
            Self::UserTemplate { key, .. } => key,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeName {
    Template(String),
    Alias(String),
    UserTemplate(String),
}

impl From<&Type> for TypeName {
    fn from(value: &Type) -> Self {
        match value {
            Type::Template { key, .. } => TypeName::Template(key.clone()),
            Type::Alias { key, .. } => TypeName::Alias(key.clone()),
            Type::UserTemplate { key, .. } => TypeName::UserTemplate(key.clone()),
        }
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::Template(name) => write(f, format_args!("{}", name)),
            TypeName::Alias(name) => write(f, format_args!("{}", name.yellow().bold())),
            TypeName::UserTemplate(name) => write(f, format_args!("{}", name.blue().bold())),
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
