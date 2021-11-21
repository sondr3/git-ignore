#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod cli;
mod config;
mod ignore;

use anyhow::Result;
use clap::{IntoApp, Parser};
use cli::{print_completion, AliasCmd, Cmds, TemplateCmd, CLI};
use colored::Colorize;
use config::Config;
use ignore::Core;

macro_rules! config_or {
    ($sel:ident, $fun:ident) => {{
        if let Some(config) = $sel.config {
            config.$fun();
        } else {
            eprintln!("{}", "No config found".bold().yellow());
        }

        return Ok(());
    }};
    ($sel:ident, $fun:ident, $name:expr) => {{
        if let Some(mut config) = $sel.config {
            config.$fun($name)?;
        } else {
            eprintln!("{}", "No config found".bold().yellow());
        }

        return Ok(());
    }};
    ($sel:ident, $fun:ident, $name:expr, $vals:expr) => {{
        if let Some(mut config) = $sel.config {
            config.$fun($name, $vals)?;
        } else {
            eprintln!("{}", "No config found".bold().yellow());
        }

        return Ok(());
    }};
}

fn main() -> Result<()> {
    let opt = CLI::parse();
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
            let mut app = CLI::into_app();
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

    if opt.update && opt.templates.is_empty() {
        return Ok(());
    }

    if opt.list {
        app.list(&opt.templates, opt.simple)?;
    } else if opt.templates.is_empty() {
        let mut app = CLI::into_app();
        app.print_help()?;
    } else {
        app.get_templates(&opt.templates, opt.simple)?;
    }

    Ok(())
}
