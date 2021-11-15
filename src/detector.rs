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
            Detector::new(
                "java",
                [
                    Matcher::by_file_name("build.gradle"),
                    Matcher::by_file_name("pom.xml"),
                ],
            ),
            Detector::new("node", [Matcher::by_file_name("package.json")]),
            Detector::new("python", [Matcher::by_file_name("requirements.txt")]),
            Detector::new(
                "haskell",
                [
                    Matcher::by_file_extension("cabal"),
                    Matcher::by_file_name("stack.yaml"),
                ],
            ),
            Detector::new("php", [Matcher::by_file_name("composer.json")]),
            Detector::new(
                "ruby",
                [
                    Matcher::by_file_extension("gemspec"),
                    Matcher::by_file_name("Gemfile"),
                ],
            ),
            Detector::new("rust", [Matcher::by_file_name("Cargo.toml")]),
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
        assert_eq!(result, vec!["php"])
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
