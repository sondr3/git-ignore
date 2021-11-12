#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use clap::{
    crate_authors, crate_description, crate_license, crate_version, AppSettings, IntoApp, Parser,
    Subcommand,
};
use colored::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(
    name = "git ignore",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!(),
    license = crate_license!(),
    global_setting = AppSettings::DeriveDisplayOrder,
)]
/// Quickly and easily add templates to .gitignore
struct Opt {
    /// List <templates> or all available templates.
    #[clap(short, long)]
    list: bool,
    /// Update templates by fetching them from gitignore.io
    #[clap(short, long)]
    update: bool,
    /// Configuration management
    #[clap(subcommand)]
    cmd: Option<Cmds>,
    /// Names of templates to show/search for
    templates: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Cmds {
    /// Manage local aliases
    #[clap(subcommand)]
    Alias(AliasCmd),
    /// Manage local templates
    #[clap(subcommand)]
    Template(TemplateCmd),
    /// Initialize configuration
    Init {
        /// Forcefully create config, possibly overwrite existing
        #[clap(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
enum AliasCmd {
    /// List available aliases
    List,
    /// Add a new alias
    Add { name: String, aliases: Vec<String> },
    /// Remove an alias
    Remove { name: String },
}

#[derive(Subcommand, Debug)]
enum TemplateCmd {
    /// List available templates
    List,
    /// Add a new template
    Add { name: String, path: PathBuf },
    /// Remove a template
    Remove { name: String },
}

#[derive(Debug)]
struct GitIgnore {
    server: String,
    cache_dir: PathBuf,
    ignore_file: PathBuf,
    config: Option<Config>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Language {
    key: String,
    name: String,
    #[serde(rename = "fileName")]
    file_name: String,
    contents: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    aliases: HashMap<String, Vec<String>>,
    templates: HashMap<String, PathBuf>,
    #[serde(skip)]
    path: PathBuf,
}

impl Config {
    fn new(path: PathBuf) -> Self {
        Self {
            aliases: Default::default(),
            templates: Default::default(),
            path,
        }
    }

    fn create(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Config::create_dir(path);

        let config = Config::new(path.to_path_buf());
        config.write()
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&self.path)?;
        file.write_all(toml::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }

    fn from_dir(path: &Path) -> Option<Self> {
        if path.exists() {
            let file = Path::new(&path);
            let file = read_to_string(file).unwrap();

            let result: Config = toml::from_str(&file).unwrap();
            Some(result)
        } else {
            None
        }
    }

    fn create_dir(path: &Path) {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("Could not create config directory");
            }
        }
    }
}

impl GitIgnore {
    /// Creates a new instance of the `git-ignore` program. Thanks to
    /// `directories` we support crossplatform caching of our results, the cache
    /// directories works on macOS, Linux and Windows. See the documentation for
    /// their locations.
    fn new() -> Self {
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

        let config_file: PathBuf = [
            proj_dir
                .config_dir()
                .to_str()
                .expect("Could not unwrap config directory"),
            "config.toml",
        ]
        .iter()
        .collect();

        let config = Config::from_dir(&config_file);

        GitIgnore {
            server: "https://www.gitignore.io/api/list?format=json".into(),
            cache_dir,
            ignore_file,
            config,
        }
    }

    /// Returns true if the cache directory or `ignore.json` file exists, false
    /// otherwise.
    fn cache_exists(&self) -> bool {
        self.cache_dir.exists() || self.ignore_file.exists()
    }

    /// Creates the cache dir if it doesn't exist.
    fn create_dirs(&self) -> std::io::Result<()> {
        if !self.cache_exists() {
            std::fs::create_dir_all(&self.cache_dir)?;
        }

        Ok(())
    }

    /// Both updates and initializes `git-ignore`. Creates the cache directory
    /// if it doesn't exist and then downloads the templates from
    /// [gitignore.io](https://www.gitignore.io), saving them in the cache
    /// directory.
    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_dirs()?;
        self.fetch_gitignore()?;

        eprintln!("{}: Update successful", "Info".bold().green());
        Ok(())
    }

    /// Fetches all the templates from [gitignore.io](http://gitignore.io/),
    /// and writes the contents to the cache for easy future retrieval.
    fn fetch_gitignore(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res = attohttpc::get(&self.server).send()?;

        let mut file = File::create(&self.ignore_file)?;
        file.write_all(&res.bytes()?)?;

        Ok(())
    }

    /// Iterates over the HashMap from `read_file` and either filters out
    /// entries not in the `names` Vector or adds all of them, finally sorting
    /// them for consistent output.
    fn get_template_names(
        &self,
        names: &[String],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let templates = self.read_file()?;
        let result = {
            let mut result: Vec<String> = Vec::new();

            for entry in templates.keys() {
                if names.is_empty() {
                    result.push(entry.to_string());
                } else {
                    for name in names {
                        if entry.starts_with(name) {
                            result.push(entry.to_string())
                        }
                    }
                }
            }

            result.sort_unstable();
            result
        };

        Ok(result)
    }

    /// Writes the `content` field for each entry in templates from `read_file`
    /// to `stdout`.
    fn get_templates(&self, names: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut templates = self.read_file()?;
        templates.retain(|k, _| names.contains(k));
        let mut result = String::new();

        for language in templates.values() {
            result.push_str(&language.contents);
        }

        if !result.is_empty() {
            let mut header = "\n\n### Created by https://www.gitignore.io".to_string();
            header.push_str(&result);
            result = header;
        }

        println!("{}", result);
        Ok(())
    }

    /// Reads the `ignore.json` and serializes it using Serde to a HashMap where
    /// the keys are each individual template and the value the contents (and
    /// some other stuff).
    fn read_file(&self) -> Result<HashMap<String, Language>, Box<dyn std::error::Error>> {
        let file = Path::new(&self.ignore_file);
        let file = read_to_string(file)?;

        let result: HashMap<String, Language> = serde_json::from_str(&file)?;
        Ok(result)
    }
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "Sondre Nilsen", "git-ignore")
        .expect("Could not find project directory.")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let app = GitIgnore::new();

    match opt.cmd {
        Some(Cmds::Init { .. }) => {
            let dirs = project_dirs();

            let config_file: PathBuf = [
                dirs.config_dir()
                    .to_str()
                    .expect("Could not unwrap config directory"),
                "config.toml",
            ]
            .iter()
            .collect();

            Config::create(&config_file)?;
            return Ok(());
        }
        Some(Cmds::Alias(cmd)) => match cmd {
            AliasCmd::List => {
                if let Some(config) = app.config {
                    for (name, aliases) in config.aliases.iter() {
                        println!("{}: {:?}", name, aliases);
                    }
                }
                return Ok(());
            }
            AliasCmd::Add { name, aliases } => {
                if let Some(mut config) = app.config {
                    config.aliases.insert(name, aliases);
                    config.write()?;
                }
                return Ok(());
            }
            AliasCmd::Remove { name } => {
                if let Some(mut config) = app.config {
                    config.aliases.remove(&name);
                    config.write()?;
                }
                return Ok(());
            }
        },
        Some(Cmds::Template(cmd)) => match cmd {
            TemplateCmd::List => {
                if let Some(config) = app.config {
                    for (name, path) in config.templates.iter() {
                        println!("{}: {:?}", name, path);
                    }
                }
                return Ok(());
            }
            TemplateCmd::Add { name, path } => {
                if let Some(mut config) = app.config {
                    config.templates.insert(name, path);
                    config.write()?;
                }
                return Ok(());
            }
            TemplateCmd::Remove { name } => {
                if let Some(mut config) = app.config {
                    config.templates.remove(&name);
                    config.write()?;
                }
                return Ok(());
            }
        },
        _ => {}
    };

    if opt.update {
        app.update()?;
    } else if !app.cache_exists() {
        eprintln!(
            "{}: Cache directory or ignore file not found, attempting update.",
            "Warning".bold().red(),
        );
        app.update()?;
    } else {
        eprintln!(
            "{}: You are using cached results, pass '-u' to update the cache\n",
            "Info".bold().green(),
        );
    }

    if opt.list {
        println!("{:#?}", app.get_template_names(&opt.templates)?);
    } else if opt.templates.is_empty() {
        let mut app = Opt::into_app();
        app.print_help()?;
    } else {
        app.get_templates(&opt.templates)?;
    }

    Ok(())
}
