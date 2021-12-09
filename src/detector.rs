use std::ffi::OsString;

#[derive(Debug)]
pub struct Detectors {
    detectors: Vec<Detector>,
}

impl Detectors {
    pub fn detects<E: DirEntry>(&self, entries: &[E]) -> Vec<String> {
        self.detectors
            .iter()
            .filter_map(|detector| detector.detects(entries))
            .collect()
    }
}

impl Default for Detectors {
    /// Based on https://github.com/starship/starship/tree/master/src/configs
    fn default() -> Self {
        let detectors = vec![
            Detector::new("crystal", [Matcher::by_file_name("shard.yml")]),
            Detector::new(
                "dart",
                [
                    Matcher::by_file_name("pubspec.yaml"),
                    Matcher::by_file_name("pubspec.yml"),
                    Matcher::by_file_name("pubspec.lock"),
                ],
            ),
            Detector::new("elixir", [Matcher::by_file_name("mix.exs")]),
            Detector::new(
                "elm",
                [
                    Matcher::by_file_name("elm.json"),
                    Matcher::by_file_name("elm-package.json"),
                    Matcher::by_file_name(".elm-version"),
                ],
            ),
            Detector::new(
                "erlang",
                [
                    Matcher::by_file_name("rebar.config"),
                    Matcher::by_file_name("erlang.mk"),
                ],
            ),
            Detector::new(
                "haskell",
                [
                    Matcher::by_file_extension("cabal"),
                    Matcher::by_file_name("stack.yaml"),
                    Matcher::by_file_name("Setup.hs"),
                ],
            ),
            Detector::new(
                "go",
                [
                    Matcher::by_file_name("go.mod"),
                    Matcher::by_file_name("go.sum"),
                    Matcher::by_file_name("glide.yaml"),
                    Matcher::by_file_name("Gopkg.yml"),
                    Matcher::by_file_name("Gopkg.lock"),
                    Matcher::by_file_name(".go-version"),
                ],
            ),
            Detector::new(
                "java",
                [
                    Matcher::by_file_name("build.gradle"),
                    Matcher::by_file_name("pom.xml"),
                    Matcher::by_file_name("build.gradle.kts"),
                    Matcher::by_file_name("build.sbt"),
                    Matcher::by_file_name(".java.version"),
                    Matcher::by_file_name("deps.edn"),
                    Matcher::by_file_name("project.clj"),
                    Matcher::by_file_name("build.boot"),
                ],
            ),
            Detector::new(
                "julia",
                [
                    Matcher::by_file_name("Project.toml"),
                    Matcher::by_file_name("Manifest.toml"),
                ],
            ),
            Detector::new("nim", [Matcher::by_file_name("nim.cfg")]),
            Detector::new(
                "node",
                [
                    Matcher::by_file_name("package.json"),
                    Matcher::by_file_name(".node-version"),
                    Matcher::by_file_name(".nvmrc"),
                ],
            ),
            Detector::new(
                "ocaml",
                [
                    Matcher::by_file_name("dune"),
                    Matcher::by_file_name("dune-project"),
                    Matcher::by_file_name("jbuild"),
                    Matcher::by_file_name("jbuild-ignore"),
                    Matcher::by_file_name(".merlin"),
                    Matcher::by_file_extension("opam"),
                ],
            ),
            Detector::new(
                "perl",
                [
                    Matcher::by_file_name("Makefile.PL"),
                    Matcher::by_file_name("Build.PL"),
                    Matcher::by_file_name("cpanfile"),
                    Matcher::by_file_name("cpanfile.snapshot"),
                    Matcher::by_file_name("META.json"),
                    Matcher::by_file_name("META.yml"),
                    Matcher::by_file_name(".perl-version"),
                ],
            ),
            Detector::new(
                "composer", // php
                [
                    Matcher::by_file_name("composer.json"),
                    Matcher::by_file_name(".php-version"),
                ],
            ),
            Detector::new(
                "purescript",
                [
                    Matcher::by_file_name("spago.dhall"),
                    Matcher::by_file_name("packages.dhall"),
                ],
            ),
            Detector::new(
                "python",
                [
                    Matcher::by_file_name("requirements.txt"),
                    Matcher::by_file_name(".python-version"),
                    Matcher::by_file_name("pyproject.toml"),
                    Matcher::by_file_name("Pipfile"),
                    Matcher::by_file_name("tox.ini"),
                    Matcher::by_file_name("setup.py"),
                    Matcher::by_file_name("__init__.py"),
                ],
            ),
            Detector::new("r", [Matcher::by_file_name(".Rprofile")]),
            Detector::new(
                "ruby",
                [
                    Matcher::by_file_extension("gemspec"),
                    Matcher::by_file_name("Gemfile"),
                    Matcher::by_file_name(".ruby-version"),
                ],
            ),
            Detector::new("rust", [Matcher::by_file_name("Cargo.toml")]),
            Detector::new(
                "scala",
                [
                    Matcher::by_file_name(".scalaenv"),
                    Matcher::by_file_name(".sbtenv"),
                    Matcher::by_file_name("build.sbt"),
                ],
            ),
            Detector::new("swift", [Matcher::by_file_name("Package.swift")]),
            Detector::new("zig", [Matcher::by_file_extension("zig")]),
        ];
        Detectors { detectors }
    }
}

#[derive(Debug)]
struct Detector {
    template: String,
    matchers: Vec<Matcher>,
}

impl Detector {
    fn new<T: Into<String>, MS: Into<Vec<Matcher>>>(template: T, matchers: MS) -> Self {
        Detector {
            template: template.into(),
            matchers: matchers.into(),
        }
    }

    fn detects<E: DirEntry>(&self, entries: &[E]) -> Option<String> {
        let result = self
            .matchers
            .iter()
            .any(|matcher| entries.iter().any(|entry| matcher.matches(entry)));
        if result {
            Some(self.template.clone())
        } else {
            None
        }
    }
}

pub trait DirEntry {
    fn file_name(&self) -> OsString;
    fn extension(&self) -> Option<OsString>;
    fn is_file(&self) -> bool;
}

impl DirEntry for std::fs::DirEntry {
    fn file_name(&self) -> OsString {
        self.file_name()
    }

    fn extension(&self) -> Option<OsString> {
        let path = self.path();
        path.extension().map(OsString::from)
    }

    fn is_file(&self) -> bool {
        let path = self.path();
        path.is_file()
    }
}

#[derive(Debug)]
enum Matcher {
    ByFileExtension(OsString),
    ByFileName(OsString),
}

impl Matcher {
    fn by_file_extension<T: Into<OsString>>(extension: T) -> Self {
        Self::ByFileExtension(extension.into())
    }

    fn by_file_name<T: Into<OsString>>(name: T) -> Self {
        Self::ByFileName(name.into())
    }

    fn matches<E: DirEntry>(&self, entry: &E) -> bool {
        match self {
            Self::ByFileName(name) => entry.is_file() && &entry.file_name() == name,
            Self::ByFileExtension(extension) => {
                entry.is_file() && entry.extension() == Some(extension.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::detector::{Detectors, DirEntry};
    use std::ffi::OsString;

    struct FakeDirEntry {
        file_name: OsString,
        extension: Option<OsString>,
        is_file: bool,
    }

    impl FakeDirEntry {
        fn new<T: Into<OsString>>(file_name: T, extension: Option<T>, is_file: bool) -> Self {
            FakeDirEntry {
                file_name: file_name.into(),
                extension: extension.map(|pe| pe.into()),
                is_file,
            }
        }
    }

    impl DirEntry for FakeDirEntry {
        fn file_name(&self) -> OsString {
            self.file_name.clone()
        }

        fn extension(&self) -> Option<OsString> {
            self.extension.clone()
        }

        fn is_file(&self) -> bool {
            self.is_file
        }
    }

    #[test]
    fn detects_java_from_build_gradle() {
        let entry = FakeDirEntry::new("build.gradle", Some("gradle"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["java"])
    }

    #[test]
    fn detects_java_from_pom_xml() {
        let entry = FakeDirEntry::new("pom.xml", Some("xml"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["java"])
    }

    #[test]
    fn detects_node_from_package_json() {
        let entry = FakeDirEntry::new("package.json", Some("json"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["node"])
    }

    #[test]
    fn detects_python_from_requirements_txt() {
        let entry = FakeDirEntry::new("requirements.txt", Some("txt"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["python"])
    }

    #[test]
    fn detects_haskell_from_dot_cabal() {
        let entry = FakeDirEntry::new("git-ignore.cabal", Some("cabal"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["haskell"])
    }

    #[test]
    fn detects_haskell_from_stack_yaml() {
        let entry = FakeDirEntry::new("stack.yaml", Some("yaml"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["haskell"])
    }

    #[test]
    fn detects_php_from_compose_json() {
        let entry = FakeDirEntry::new("composer.json", Some("json"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["composer"])
    }

    #[test]
    fn detects_ruby_from_dot_gemspec() {
        let entry = FakeDirEntry::new("git-ignore.gemspec", Some("gemspec"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["ruby"])
    }

    #[test]
    fn detects_ruby_from_gemfile() {
        let entry = FakeDirEntry::new("Gemfile", None, true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["ruby"])
    }

    #[test]
    fn detects_rust_from_cargo_toml() {
        let entry = FakeDirEntry::new("Cargo.toml", Some("toml"), true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["rust"])
    }
}
