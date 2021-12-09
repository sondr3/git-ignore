use clap::{App, AppSettings, Parser, Subcommand};
use clap_generate::{generate, Generator, Shell};
use std::io;

#[derive(Parser, Debug)]
#[clap(
    name = "git ignore",
    about,
    version,
    author,
    global_setting = AppSettings::DeriveDisplayOrder,
    global_setting = AppSettings::ArgsNegateSubcommands,
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
    /// Ignore all user defined aliases and templates
    #[clap(short, long)]
    pub simple: bool,
    /// Autodetect templates based on the existing files
    #[clap(short, long)]
    pub auto: bool,
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
    /// Initialize user configuration
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
    #[clap(visible_alias = "ls")]
    List,
    /// Add a new alias
    Add { name: String, aliases: Vec<String> },
    /// Remove an alias
    #[clap(visible_alias = "rm")]
    Remove { name: String },
}

#[derive(Subcommand, Debug)]
pub enum TemplateCmd {
    /// List available templates
    #[clap(visible_alias = "ls")]
    List,
    /// Add a new template
    Add { name: String, file_name: String },
    /// Remove a template
    #[clap(visible_alias = "rm")]
    Remove { name: String },
}

pub fn print_completion<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
