use clap::{ArgEnum, IntoApp};
use clap_generate::generate_to;
use man::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process,
};

include!("src/cli.rs");

fn main() {
    let man = Manual::new("git-ignore")
        .about("Quickly and easily list and fetch .gitignore templates from www.gitignore.io")
        .description(
            "git-ignore is a small utility to quickly create or add templates from \
www.gitignore.io to your .gitignore files. The main difference to other similar utilities is that \
it works offline, it does this by caching all available templates and storing them in your \
$HOME/.cache directory. It is also reliable and efficient, thanks to Rust.",
        )
        .author(Author::new("Sondre Nilsen").email("nilsen.sondre@gmail.com"))
        .flag(
            Flag::new()
                .short("-h")
                .long("--help")
                .help("Show short help or these man pages."),
        )
        .flag(
            Flag::new()
                .short("-l")
                .long("--list")
                .help("List TEMPLATES or all available templates."),
        )
        .flag(
            Flag::new()
                .short("-u")
                .long("--update")
                .help("Update the local cache from www.gitignore.io."),
        )
        .flag(
            Flag::new()
                .short("-V")
                .long("--version")
                .help("Print version of git-ignore."),
        )
        .arg(Arg::new("TEMPLATES"))
        .example(
            Example::new()
                .text("List all available templates")
                .prompt("$")
                .command("git ignore [-l/--list]")
                .output("[ list of all templates ]"),
        )
        .example(
            Example::new()
                .text("List all matching TEMPLATES")
                .prompt("$")
                .command("git ignore [-l/--list] rust intellij")
                .output("[ rust intellij intellij+all intellij+iml ]"),
        )
        .example(
            Example::new()
                .text("Print matching templates to STDOUT")
                .prompt("$")
                .command("git ignore rust intellij")
                .output("### RUST ### [...]"),
        )
        .custom(
            Section::new("Usage notes")
                .paragraph(
                    "If the required `ignore.json` file does not exist, an attempt \
will be made to download it. This requires an internet connection, but only once.",
                )
                .paragraph(
                    "The program will print a small notice when you are only using \
cached templates, this is printed to STDERR so it will not interfere with piping etc.",
                )
                .paragraph(
                    "Note that listing templates doesn't require exact matches while \
printing the template does. When listing it matches any template starting with each query, i.e. \
`intellij` matches all templates starting with `intellij` (see example below).",
                ),
        )
        .render();

    // OUT_DIR is set by Cargo and it's where any additional build artifacts
    // are written.
    let out_dir = if let Some(out_dir) = env::var_os("OUT_DIR") {
        out_dir
    } else {
        eprintln!("Oh no");
        process::exit(1);
    };

    let out_path = PathBuf::from(&out_dir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    fs::create_dir_all(&path).unwrap();

    let mut app = CLI::into_app();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, "git-ignore", &path).unwrap();
    }

    let file = Path::new(&path).join("git-ignore.1");
    File::create(&file)
        .expect("Couldn't open man pages")
        .write_all(man.as_bytes())
        .expect("Unable to write man page.");
}
