#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod cli;
mod config;
mod ignore;

use clap::{IntoApp, Parser};
use cli::{print_completion, AliasCmd, Cmds, TemplateCmd, CLI};
use colored::*;
use config::Config;
use ignore::{project_dirs, GitIgnore};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = CLI::parse();
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
                    config.add_alias(name, aliases)?;
                }
                return Ok(());
            }
            AliasCmd::Remove { name } => {
                if let Some(mut config) = app.config {
                    config.remove_alias(name)?;
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
                    config.add_template(name, path)?;
                }
                return Ok(());
            }
            TemplateCmd::Remove { name } => {
                if let Some(mut config) = app.config {
                    config.remove_template(name)?;
                }
                return Ok(());
            }
        },
        Some(Cmds::Completion { shell }) => {
            let mut app = CLI::into_app();
            print_completion(shell, &mut app);
            return Ok(());
        }
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
        let mut app = CLI::into_app();
        app.print_help()?;
    } else {
        app.get_templates(&opt.templates)?;
    }

    Ok(())
}
