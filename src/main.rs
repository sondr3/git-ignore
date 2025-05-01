#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod cli;
mod config;
mod detector;
mod ignore;

use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, Write},
};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{AliasCmd, Cli, Cmds, TemplateCmd, print_completion};
use colored::Colorize;
use config::Config;
use ignore::Core;

macro_rules! config_or {
    ($sel:ident, $fun:ident) => {{
        if let Some(config) = $sel.config {
            config.$fun();
        } else {
            eprintln!(
                "{}",
                "No config found, run `git ignore init` to create it."
                    .bold()
                    .yellow()
            );
        }

        return Ok(());
    }};
    ($sel:ident, $fun:ident, $name:expr) => {{
        if let Some(mut config) = $sel.config {
            config.$fun($name)?;
        } else {
            eprintln!(
                "{}",
                "No config found, run `git ignore init` to create it."
                    .bold()
                    .yellow()
            );
        }

        return Ok(());
    }};
    ($sel:ident, $fun:ident, $name:expr, $vals:expr) => {{
        if let Some(mut config) = $sel.config {
            config.$fun($name, $vals)?;
        } else {
            eprintln!(
                "{}",
                "No config found, run `git ignore init` to create it."
                    .bold()
                    .yellow()
            );
        }

        return Ok(());
    }};
}

fn main() -> Result<()> {
    let opt = Cli::parse();
    let app = Core::new();

    match opt.cmd {
        Some(Cmds::Init { force }) => return Config::create(force),
        Some(Cmds::Alias(cmd)) => match cmd {
            AliasCmd::List => config_or!(app, list_aliases),
            AliasCmd::Add { name, aliases } => config_or!(app, add_alias, name, aliases),
            AliasCmd::Remove { name } => config_or!(app, remove_alias, &name),
        },
        Some(Cmds::Template(cmd)) => match cmd {
            TemplateCmd::List => config_or!(app, list_templates),
            TemplateCmd::Add { name, file_name } => config_or!(app, add_template, name, file_name),
            TemplateCmd::Remove { name } => config_or!(app, remove_template, &name),
        },
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
        app.list(templates.as_slice(), opt.simple)?
    } else if templates.is_empty() {
        let mut app = Cli::command();
        app.render_help().to_string()
    } else {
        app.get_templates(templates.as_slice(), opt.simple)?
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
