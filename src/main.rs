#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod cli;
mod data;
mod detector;
mod ignore;
mod user_data;

use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, Write},
};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{AliasCmd, Cli, Cmds, TemplateCmd, print_completion};
use colored::Colorize;
use ignore::Core;
use user_data::UserData;

use crate::data::IgnoreData;

fn main() -> Result<()> {
    let opt = Cli::parse();
    let app = Core::new()?;
    let mut user_data = UserData::new()?;
    let ignore_data = IgnoreData::new(&user_data)?;

    match opt.cmd {
        Some(Cmds::Init { force }) => return UserData::create(force),
        Some(Cmds::Alias(cmd)) => {
            return match cmd {
                AliasCmd::List => {
                    ignore_data.list_aliases();
                    return Ok(());
                }
                AliasCmd::Add { name, aliases } => user_data.add_alias(name, aliases),
                AliasCmd::Remove { name } => user_data.remove_alias(&name),
            };
        }
        Some(Cmds::Template(cmd)) => {
            return match cmd {
                TemplateCmd::List => {
                    ignore_data.list_templates();
                    return Ok(());
                }
                TemplateCmd::Add { name, file_name } => user_data.add_template(name, file_name),
                TemplateCmd::Remove { name } => user_data.remove_template(&name),
            };
        }
        Some(Cmds::Completion { shell }) => {
            let mut app = Cli::command();
            print_completion(shell, &mut app);
            return Ok(());
        }
        _ => {}
    };

    if opt.update {
        app.update()?;
    } else if app.cache_exists() {
        eprintln!(
            "{}: You are using cached results, pass '-u' to update the cache\n",
            "Info".bold().green(),
        );
    } else {
        eprintln!(
            "{}: Cache directory or ignore file not found, attempting update.",
            "Warning".bold().red(),
        );
        app.update()?;
    }

    let mut all_templates: HashSet<String> = opt.templates.into_iter().collect();
    if opt.auto {
        for template in app.autodetect_templates()? {
            all_templates.insert(template);
        }
    }

    let templates: Vec<String> = all_templates.iter().cloned().collect();

    if opt.update && templates.is_empty() {
        return Ok(());
    }

    let str = if opt.list {
        app.list(&ignore_data, templates.as_slice())?
    } else if templates.is_empty() {
        let mut app = Cli::command();
        app.render_help().to_string()
    } else {
        app.get_templates(&ignore_data, templates.as_slice())?
    };

    if opt.write {
        let file = std::env::current_dir()?.join(".gitignore");
        if !file.exists() {
            eprintln!(
                "{}: no '.gitignore' file found, creating...",
                "Info".bold().green()
            );
            let mut file = File::create(&file)?;
            file.write_all(str.as_bytes())?;
        } else if file.exists() && !opt.force {
            eprintln!(
                "{}: '.gitignore' already exists, use '-f' to force write",
                "Warning".bold().red()
            );
        } else if file.exists() && opt.force {
            eprintln!(
                "{}: appending results to '.gitignore'",
                "Info".bold().green()
            );
            let mut file = OpenOptions::new().append(true).open(&file)?;
            file.write_all(str.as_bytes())?;
        }
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(str.as_bytes())?;
    }

    Ok(())
}
