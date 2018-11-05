#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]

extern crate reqwest;
extern crate structopt;

use std::io::Read;
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
    List {
        #[structopt(raw(required = "false"))]
        templates: Vec<String>,
    },
    #[structopt(name = "get")]
    /// Get listed templates
    Get { templates: Vec<String> },
}

fn gitignore_list(templates: &[String]) -> Result<(), Box<std::error::Error>> {
    let url = "https://www.gitignore.io/api/list";
    let mut res = reqwest::get(url)?;

    let all = templates.is_empty();

    let mut response = Vec::new();
    res.read_to_end(&mut response)?;
    let response = String::from_utf8(response)?;
    let response = {
        let tmp = response.replace("\n", ",");
        let tmp = tmp.split(',');
        let mut list: Vec<String> = Vec::new();

        for entry in tmp {
            if all {
                list.push(entry.to_string());
            } else {
                for item in templates {
                    if entry.to_string().starts_with(item) {
                        list.push(entry.to_string());
                    }
                }
            }
        }

        list
    };
    println!("{:#?}", response);
    println!("{:?}", templates);
    println!("{:?}", all);

    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    match Opt::from_args() {
        Opt::List { templates } => gitignore_list(&templates)?,
        Opt::Get { templates: _ } => println!("Get some shit"),
    }
    Ok(())
}
