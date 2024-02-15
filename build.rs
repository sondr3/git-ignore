use clap::{CommandFactory, ValueEnum};
use clap_complete::generate_to;
use clap_mangen::Man;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Error,
    io::Write,
    path::{Path, PathBuf},
};

include!("src/cli.rs");

#[derive(Debug, Serialize, Deserialize)]
pub struct Detector {
    detect_files: Vec<String>,
    detect_extensions: Vec<String>,
    detect_folders: Vec<String>,
}

fn collect_detectors(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::current_dir().unwrap().join("data");
    let json = dir.join("parsed.json");
    let content = std::fs::read_to_string(json)?;
    let detector: HashMap<String, Detector> = serde_json::from_str(&content)?;

    let json = dir.join("aliases.json");
    let content = std::fs::read_to_string(json)?;
    let aliases: HashMap<String, String> = serde_json::from_str(&content)?;

    let res: HashMap<String, Detector> = detector
        .into_iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            if let Some(alias) = aliases.get(&k) {
                acc.insert(alias.clone(), v);
            } else {
                acc.insert(k.clone(), v);
            }
            acc
        })
        .into_iter()
        .filter(|(_, v)| {
            !v.detect_files.is_empty()
                || !v.detect_extensions.is_empty()
                || !v.detect_folders.is_empty()
        })
        .collect();

    let mut output = File::create(out_dir.join("detectors.rs"))?;
    writeln!(
        output,
        r#"
        fn detectors() -> Vec<Detector> {{
            vec![
            "#
    )?;
    for (lang, detection) in res {
        let matchers = detection
            .detect_files
            .into_iter()
            .map(|file| {
                quote! { Matcher::FileName(OsString::from(#file)) }
            })
            .chain(detection.detect_extensions.into_iter().map(|ext| {
                quote! { Matcher::FileExtension(OsString::from(#ext)) }
            }))
            .chain(detection.detect_folders.into_iter().map(|folder| {
                quote! { Matcher::DirName(OsString::from(#folder)) }
            }))
            .collect::<Vec<_>>();

        let detector_code = quote! {
            Detector {
                template: String::from(#lang),
                matchers: vec![#(#matchers),*],
            },
        };

        writeln!(output, "{}", detector_code)?;
    }

    writeln!(
        output,
        r#"
            ]
        }}
        "#
    )?;

    Ok(())
}

fn build_shell_completion(outdir: &Path) -> Result<(), Error> {
    let mut app = Cli::command();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, "git-ignore", outdir)?;
    }

    Ok(())
}

fn build_manpages(outdir: &Path) -> Result<(), Error> {
    let app = Cli::command();

    let file = outdir.join("git-ignore.1");
    let mut file = File::create(file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man");

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let out_path = PathBuf::from(outdir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    build_shell_completion(&path)?;
    build_manpages(&path)?;

    collect_detectors(&out_path)?;

    Ok(())
}
