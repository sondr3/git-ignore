use clap::{ArgEnum, IntoApp};
use clap_generate::generate_to;
use std::env::current_dir;

include!("src/cli.rs");

fn main() {
    let out_path = current_dir().unwrap().join("assets");

    let mut app = CLI::into_app();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, "git-ignore", &out_path).unwrap();
    }
}
