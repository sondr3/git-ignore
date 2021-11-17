use crate::config::Config;
use colored::Colorize;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{write, Display},
    fs::{read_to_string, File},
    hash::{Hash, Hasher},
    io::Write,
    path::{Path, PathBuf},
};

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "Sondre Nilsen", "git-ignore")
        .expect("Could not find project directory.")
}

#[derive(Debug)]
pub struct Core {
    server: String,
    cache_dir: PathBuf,
    ignore_file: PathBuf,
    pub config: Option<Config>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Language {
    key: String,
    name: String,
    #[serde(rename = "fileName")]
    file_name: String,
    contents: String,
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

    fn contains(&self, name: &str) -> bool {
        let inner = self.inner();
        inner.contains(name)
    }
}

impl Core {
    /// Creates a new instance of the `git-ignore` program. Thanks to
    /// `directories` we support crossplatform caching of our results, the cache
    /// directories works on macOS, Linux and Windows. See the documentation for
    /// their locations.
    pub fn new() -> Self {
        let proj_dir = project_dirs();
        let cache_dir: PathBuf = proj_dir.cache_dir().into();
        let ignore_file: PathBuf = [
            cache_dir
                .to_str()
                .expect("Could not unwrap cache directory."),
            "ignore.json",
        ]
        .iter()
        .collect();

        let config = Config::from_dir();

        Core {
            server: "https://www.gitignore.io/api/list?format=json".into(),
            cache_dir,
            ignore_file,
            config,
        }
    }

    /// Both updates and initializes `git-ignore`. Creates the cache directory
    /// if it doesn't exist and then downloads the templates from
    /// [gitignore.io](https://www.gitignore.io), saving them in the cache
    /// directory.
    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_dirs()?;
        self.fetch_gitignore()?;

        eprintln!("{}: Update successful", "Info".bold().green());
        Ok(())
    }

    pub fn list(&self, names: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let templates = self.all_names()?;
        let mut result = if names.is_empty() {
            templates.into_iter().collect::<Vec<_>>()
        } else {
            let mut result = Vec::new();

            for entry in templates {
                for name in names {
                    if entry.contains(name) {
                        result.push(entry.clone());
                    }
                }
            }
            result
        };

        result.sort_unstable();

        for entry in result {
            println!("  {}", entry);
        }

        Ok(())
    }

    /// Writes the `content` field for each entry in templates from `read_file`
    /// to `stdout`.
    pub fn get_templates(&self, names: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let (aliases, templates) = match &self.config {
            Some(config) => (config.aliases.clone(), config.templates.clone()),
            None => (HashMap::new(), HashMap::new()),
        };

        let ignore_file = self.read_file()?;
        let mut result = String::new();

        for name in names {
            if let Some(val) = templates.get(name) {
                let template = Config::read_template(val)?;
                result.push_str(&template);
            } else if let Some(val) = aliases.get(name) {
                for alias in val {
                    if let Some(language) = ignore_file.get(&Type::Alias(alias.to_string())) {
                        result.push_str(&language.contents);
                    }
                }
            } else if let Some(language) = ignore_file.get(&Type::Normal(name.to_string())) {
                result.push_str(&language.contents);
            }
        }

        if !result.is_empty() {
            let mut header = "\n\n### Created by https://www.gitignore.io".to_string();
            header.push_str(&result);
            result = header;
        }

        println!("{}", result);
        Ok(())
    }

    fn all_names(&self) -> Result<HashSet<Type>, Box<dyn std::error::Error>> {
        let templates = self.read_file()?;
        let config_names = match &self.config {
            Some(config) => config.names(),
            _ => vec![],
        };

        let mut combined: HashSet<Type> = config_names.into_iter().collect();
        combined.extend(templates.keys().cloned());

        Ok(combined)
    }

    /// Fetches all the templates from [gitignore.io](http://gitignore.io/),
    /// and writes the contents to the cache for easy future retrieval.
    fn fetch_gitignore(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res = attohttpc::get(&self.server).send()?;

        let mut file = File::create(&self.ignore_file)?;
        file.write_all(&res.bytes()?)?;

        Ok(())
    }

    /// Returns true if the cache directory or `ignore.json` file exists, false
    /// otherwise.
    pub fn cache_exists(&self) -> bool {
        self.cache_dir.exists() || self.ignore_file.exists()
    }

    /// Creates the cache dir if it doesn't exist.
    fn create_dirs(&self) -> std::io::Result<()> {
        if !self.cache_exists() {
            std::fs::create_dir_all(&self.cache_dir)?;
        }

        Ok(())
    }

    /// Reads the `ignore.json` and serializes it using Serde to a `HashMap` where
    /// the keys are each individual template and the value the contents (and
    /// some other stuff).
    fn read_file(&self) -> Result<HashMap<Type, Language>, Box<dyn std::error::Error>> {
        let file = Path::new(&self.ignore_file);
        let file = read_to_string(file)?;

        let result: HashMap<String, Language> = serde_json::from_str(&file)?;
        let result: HashMap<Type, Language> = result
            .into_iter()
            .map(|(k, v)| (Type::Normal(k), v))
            .collect();

        Ok(result)
    }
}
