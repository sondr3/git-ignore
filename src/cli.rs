use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Parser, Subcommand};
use clap_generate::{generate, Generator, Shell};
use std::{io, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(
    name = "git ignore",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!(),
    global_setting = AppSettings::DeriveDisplayOrder,
)]
#[allow(clippy::upper_case_acronyms)]
/// Quickly and easily add templates to .gitignore
pub struct CLI {
    /// List <templates> or all available templates.
    #[clap(short, long)]
    pub list: bool,
    /// Update templates by fetching them from gitignore.io
    #[clap(short, long)]
    pub update: bool,
    /// Configuration management
    #[clap(subcommand)]
    pub cmd: Option<Cmds>,
    /// Names of templates to show/search for
    pub templates: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Cmds {
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
    /// Generate shell completion
    Completion {
        /// Shell to generate completion for
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
pub enum AliasCmd {
    /// List available aliases
    List,
    /// Add a new alias
    Add { name: String, aliases: Vec<String> },
    /// Remove an alias
    Remove { name: String },
}

#[derive(Subcommand, Debug)]
pub enum TemplateCmd {
    /// List available templates
    List,
    /// Add a new template
    Add { name: String, path: PathBuf },
    /// Remove a template
    Remove { name: String },
}

pub fn print_completion<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
