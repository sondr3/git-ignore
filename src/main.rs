//! # `git ignore`
//!
//! `git-ignore-generator` is a simple and easy to use way to quickly and
//! effortlessly list and grab `.gitignore` templates from
//! [gitignore.io](https://www.gitignore.io/) from the command line.
//!
//! ## What and why
//!
//! Far too often I find myself going to
//! [gitignore.io](https://www.gitignore.io/) to quickly get `.gitignore`
//! templates for my projects, so what would any reasonable programmer do for
//! menial and repetitive tasks? [Automate](https://xkcd.com/1319/)
//! [it](https://xkcd.com/1205/)! Now you can quickly and easily get and list
//! all the templates available on gitignore.io.
//!
//! # Installation
//!
//! Make sure you have Rust installed (I recommend installing via
//! [rustup](https://rustup.rs/)), then run `cargo install
//! git-ignore-generator`.
//!
//! To list all the available templates:
//!
//! ```sh
//! $ git ignore --list
//! [
//! "1c",
//! "1c-bitrix",
//! "a-frame",
//! "actionscript",
//! "ada",
//! [...],
//! "zukencr8000"
//! ]
//! ```
//!
//! You can also search for templates (`--list` can be both before and after the
//! queries):
//!
//! ```sh
//! $ git ignore rust intellij --list
//! [
//! "intellij",
//! "intellij+all",
//! "intellij+iml",
//! "rust"
//! ]
//! ```
//!
//! Then you can download the templates by omitting `--list`:
//!
//! ```sh
//! $ git ignore rust intellij+all
//!
//! # Created by https://www.gitignore.io/api/rust,intellij+all
//! # Edit at https://www.gitignore.io/?templates=rust,intellij+all
//!
//! [...]
//!
//! # These are backup files generated by rustfmt
//! **/*.rs.bk
//!
//! # End of https://www.gitignore.io/api/rust,intellij+all
//! ```
//!
//! Finally, if need be, you can always run `git ignore -h` to see more options
//! --- spoiler alert, there are none.
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation
)]

use directories::ProjectDirs;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "git ignore",
    about = "Quickly and easily add templates to .gitignore",
    raw(global_settings = "&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp]")
)]
struct Opt {
    /// List available .gitignore templates
    #[structopt(short, long)]
    list: bool,
    /// Update templates from gitignore.io
    #[structopt(short, long)]
    update: bool,
    /// List of .gitignore templates to fetch/list
    #[structopt(raw(required = "false"))]
    templates: Vec<String>,
}

#[derive(Debug)]
struct GitIgnore {
    server: String,
    cache_dir: PathBuf,
    ignore_file: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct Language {
    key: String,
    name: String,
    #[serde(rename = "fileName")]
    file_name: String,
    contents: String,
}

impl GitIgnore {
    fn new() -> Self {
        let proj_dir = ProjectDirs::from("com", "sondr3", "git-ignore")
            .expect("Could not find project directory.");

        let cache_dir: PathBuf = proj_dir.cache_dir().into();
        let ignore_file: PathBuf = [
            cache_dir
                .to_str()
                .expect("Could not unwrap cache directory."),
            "ignore.json",
        ]
        .iter()
        .collect();

        GitIgnore {
            server: "https://www.gitignore.io/api/list?format=json".into(),
            cache_dir,
            ignore_file,
        }
    }

    fn create_cache_dir(&self) -> std::io::Result<()> {
        if !self.cache_dir.exists() {
            std::fs::create_dir(&self.cache_dir)?;
        }
        Ok(())
    }

    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_cache_dir()?;
        self.fetch_gitignore()?;

        Ok(())
    }

    fn fetch_gitignore(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut res = reqwest::get(&self.server)?;

        let mut response = Vec::new();
        res.read_to_end(&mut response)?;
        let response = String::from_utf8(response)?;
        let response = {
            let mut list: Vec<String> = Vec::new();
            for line in response.lines() {
                list.push(line.to_string());
            }

            list
        };

        let mut file = File::create(&self.ignore_file)?;
        for entry in response {
            writeln!(file, "{}", entry)?;
        }

        Ok(())
    }

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

            result
        };

        Ok(result)
    }

    fn get_templates(&self, names: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut templates = self.read_file()?;
        templates.retain(|k, _| names.contains(k));
        let mut result = String::new();

        for language in templates.values() {
            result.push_str(&language.contents);
        }

        println!("{}", result);
        Ok(())
    }

    fn read_file(&self) -> Result<HashMap<String, Language>, Box<dyn std::error::Error>> {
        let file = File::open(&self.ignore_file)?;
        let file: String = BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not read line."))
            .collect();

        let result: HashMap<String, Language> = serde_json::from_str(&file)?;
        Ok(result)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let app = GitIgnore::new();
    if opt.update {
        app.update()?;
    }

    if opt.list {
        println!("{:#?}", app.get_template_names(&opt.templates)?);
    } else {
        app.get_templates(&opt.templates)?;
    }

    Ok(())
}
