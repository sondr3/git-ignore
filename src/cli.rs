use clap::{Command, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(Parser, Debug)]
#[clap(name = "git-ignore", about, version, author)]
#[clap(args_conflicts_with_subcommands = true)]
/// Quickly and easily add templates to .gitignore
pub struct Cli {
    /// List <templates> or all available templates.
    #[arg(short, long)]
    pub list: bool,
    /// Update templates by fetching them from gitignore.io
    #[arg(short, long)]
    pub update: bool,
    /// Ignore all user defined aliases and templates
    #[arg(short, long)]
    pub simple: bool,
    /// Autodetect templates based on the existing files
    #[arg(short, long)]
    pub auto: bool,
    /// Configuration management
    #[command(subcommand)]
    pub cmd: Option<Cmds>,
    /// Names of templates to show/search for
    pub templates: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Cmds {
    #[command(subcommand)]
    Alias(AliasCmd),
    #[command(subcommand)]
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
        #[clap(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
/// Manage local templates
pub enum AliasCmd {
    /// List available aliases
    #[command(visible_alias = "ls")]
    List,
    /// Add a new alias
    Add { name: String, aliases: Vec<String> },
    /// Remove an alias
    #[command(visible_alias = "rm")]
    Remove { name: String },
}

#[derive(Subcommand, Debug)]
/// Manage local aliases
pub enum TemplateCmd {
    /// List available templates
    #[command(visible_alias = "ls")]
    List,
    /// Add a new template
    Add { name: String, file_name: String },
    /// Remove a template
    #[command(visible_alias = "rm")]
    Remove { name: String },
}

pub fn print_completion<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
