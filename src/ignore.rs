use std::{
    collections::HashSet,
    env::current_dir,
    fmt::Write,
    fs::{DirEntry, File, read_dir},
    io::Write as _,
    path::PathBuf,
    sync::LazyLock,
};

use anyhow::Result;
use colored::Colorize;
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};

use crate::{
    config::Config,
    data::{IgnoreData, Type, TypeName},
    detector::Detectors,
};

#[cfg(target_os = "windows")]
pub static PROJECT_DIRS: LazyLock<etcetera::app_strategy::Windows> = LazyLock::new(|| {
    choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "Sondre Aasemoen".to_string(),
        app_name: "git-ignore".to_string(),
    })
    .expect("Could not find project directory.")
});

#[cfg(not(target_os = "windows"))]
pub static PROJECT_DIRS: LazyLock<etcetera::app_strategy::Xdg> = LazyLock::new(|| {
    choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "Sondre Aasemoen".to_string(),
        app_name: "git-ignore".to_string(),
    })
    .expect("Could not find project directory.")
});

#[derive(Debug)]
pub struct Core {
    server: String,
    cache_dir: PathBuf,
    ignore_file: PathBuf,
    detectors: Detectors,
    pub config: Config,
}

impl Core {
    /// Creates a new instance of the `git-ignore` program. Thanks to
    /// `directories` we support crossplatform caching of our results, the cache
    /// directories works on macOS, Linux and Windows. See the documentation for
    /// their locations.
    pub fn new() -> Result<Self> {
        let cache_dir = PROJECT_DIRS.cache_dir();
        let ignore_file = cache_dir.join("ignore.json");
        let config = Config::new()?;

        Ok(Core {
            server: "https://www.gitignore.io/api/list?format=json".into(),
            cache_dir,
            ignore_file,
            detectors: Detectors::default(),
            config,
        })
    }

    /// Both updates and initializes `git-ignore`. Creates the cache directory
    /// if it doesn't exist and then downloads the templates from
    /// [gitignore.io](https://www.gitignore.io), saving them in the cache
    /// directory.
    pub fn update(&self) -> Result<()> {
        self.create_dirs()?;
        self.fetch_gitignore()?;

        eprintln!("{}: Update successful", "Info".bold().green());
        Ok(())
    }

    pub fn list(&self, names: &[String], simple: bool) -> Result<String> {
        let templates = self.all_names(simple)?;
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

        let result = result.into_iter().fold(String::new(), |mut s, r| {
            writeln!(s, "  {}", r).unwrap();
            s
        });

        Ok(result)
    }

    /// Creates a formatted string of all the configured templates
    pub fn get_templates(&self, names: &[String]) -> Result<String> {
        let data = IgnoreData::new(&self.ignore_file, &self.config)?;
        let mut result = String::new();

        for name in names {
            if let Some(val) = data.get_user_template(name) {
                result.push_str(&val);
            } else if let Some(val) = data.get_alias(name) {
                for alias in val {
                    if let Some(val) = data.get_user_template(&alias) {
                        result.push_str(&val);
                    } else if let Some(language) = data.get_template(&alias) {
                        result.push_str(&language);
                    } else {
                        eprintln!("{}: No such alias", name.bold().yellow());
                    }
                }
            } else if let Some(language) = data.get_template(name) {
                result.push_str(&language);
            }
        }

        if !result.is_empty() {
            let mut header = "\n\n### Created by https://www.gitignore.io".to_string();
            header.push_str(&result);
            result = header;
        }

        Ok(result)
    }

    pub fn get_templates_simple(&self, names: &[String]) -> Result<String> {
        let data = IgnoreData::new(&self.ignore_file, &self.config)?;
        let mut result = String::new();

        for name in names {
            if let Some(language) = data.get_template(name) {
                result.push_str(&language);
            }
        }

        if !result.is_empty() {
            let mut header = "\n\n### Created by https://www.gitignore.io".to_string();
            header.push_str(&result);
            result = header;
        }

        Ok(result)
    }

    pub fn autodetect_templates(&self) -> Result<Vec<String>> {
        let entries: Vec<DirEntry> = read_dir(current_dir()?)?.map(Result::unwrap).collect();
        Ok(self.detectors.detects(entries.as_slice()))
    }

    fn all_names(&self, simple: bool) -> Result<HashSet<TypeName>> {
        let data = IgnoreData::new(&self.ignore_file, &self.config)?;

        let keys = data
            .data
            .iter()
            .map(|v| match v {
                Type::Template { key, .. } => TypeName::Template(key.clone()),
                Type::Alias { key, .. } => TypeName::Alias(key.clone()),
                Type::UserTemplate { key, .. } => TypeName::UserTemplate(key.clone()),
            })
            .collect();

        if simple {
            return Ok(keys);
        }

        let mut combined: HashSet<TypeName> = self.config.names().into_iter().collect();
        combined.extend(keys);

        Ok(combined)
    }

    /// Fetches all the templates from [gitignore.io](http://gitignore.io/),
    /// and writes the contents to the cache for easy future retrieval.
    fn fetch_gitignore(&self) -> Result<()> {
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
}
