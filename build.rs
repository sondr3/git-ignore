use man::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs, process};

fn main() {
    let man = Manual::new("git ignore")
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
    let out_dir = match env::var_os("OUT_DIR") {
        Some(out_dir) => out_dir,
        None => {
            eprintln!("Oh no");
            process::exit(1);
        }
    };
    fs::create_dir_all(&out_dir).unwrap();

    let file = Path::new(&out_dir).join("git-ignore.1");
    File::create(&file)
        .expect("Couldn't open man pages")
        .write_all(man.as_bytes())
        .expect("Unable to write man page.");
}
