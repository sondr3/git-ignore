#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]

extern crate reqwest;
#[macro_use]
extern crate structopt;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
name = "git ignore",
about = "Quickly and easily add templates to .gitignore",
raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
enum Opt {
    #[structopt(name = "list")]
    /// List all available .gitignore templates
    List,
    #[structopt(name = "get")]
    /// Get listed templates
    Get,
}

fn main() {
    match Opt::from_args() {
        Opt::List => println!("List some shit"),
        Opt::Get => println!("Get some shit"),
    }
}
