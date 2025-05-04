use std::{
    env::current_dir,
    fs::{DirEntry, File, read_dir},
    io::Write as _,
    sync::LazyLock,
};

use anyhow::Result;
use colored::Colorize;
use etcetera::{AppStrategyArgs, choose_app_strategy};

use crate::{
    data::{CACHE_DIR, CACHE_FILE},
    detector::Detectors,
};

#[cfg(target_os = "windows")]
pub static PROJECT_DIRS: LazyLock<etcetera::app_strategy::Windows> = LazyLock::new(|| {
    choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "Sondre Aasemoen".to_string(),
        app_name: "git-ignore".to_string(),
    })
    .expect("Could not find project directory.")
});

#[cfg(not(target_os = "windows"))]
pub static PROJECT_DIRS: LazyLock<etcetera::app_strategy::Xdg> = LazyLock::new(|| {
    choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "Sondre Aasemoen".to_string(),
        app_name: "git-ignore".to_string(),
    })
    .expect("Could not find project directory.")
});

#[derive(Debug)]
pub struct Core {
    server: String,
    detectors: Detectors,
}

impl Core {
    /// Creates a new instance of the `git-ignore` program. Thanks to
    /// `directories` we support crossplatform caching of our results, the cache
    /// directories works on macOS, Linux and Windows. See the documentation for
    /// their locations.
    pub fn new() -> Self {
        Core {
            server: "https://www.gitignore.io/api/list?format=json".into(),
            detectors: Detectors::default(),
        }
    }

    /// Both updates and initializes `git-ignore`. Creates the cache directory
    /// if it doesn't exist and then downloads the templates from
    /// [gitignore.io](https://www.gitignore.io), saving them in the cache
    /// directory.
    pub fn update(&self) -> Result<()> {
        create_cache()?;
        self.fetch_gitignore()?;

        eprintln!("{}: Update successful", "Info".bold().green());
        Ok(())
    }

    /// Creates a formatted string of all the configured templates
    pub fn autodetect_templates(&self) -> Result<Vec<String>> {
        let entries: Vec<DirEntry> = read_dir(current_dir()?)?.map(Result::unwrap).collect();
        Ok(self.detectors.detects(entries.as_slice()))
    }

    /// Fetches all the templates from [gitignore.io](http://gitignore.io/),
    /// and writes the contents to the cache for easy future retrieval.
    fn fetch_gitignore(&self) -> Result<()> {
        let res = attohttpc::get(&self.server).send()?;

        let mut file = File::create(CACHE_FILE.as_path())?;
        file.write_all(&res.bytes()?)?;

        Ok(())
    }
}

pub fn cache_exists() -> bool {
    CACHE_DIR.exists() || CACHE_FILE.exists()
}

fn create_cache() -> std::io::Result<()> {
    if !cache_exists() {
        std::fs::create_dir_all(CACHE_DIR.as_path())?;
    }

    Ok(())
}
