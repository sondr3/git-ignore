use std::ffi::OsString;

include!(concat!(env!("OUT_DIR"), "/detectors.rs"));

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
    fn default() -> Self {
        Self {
            detectors: detectors(),
        }
    }
}

#[derive(Debug)]
struct Detector {
    template: String,
    matchers: Vec<Matcher>,
}

impl Detector {
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
    fn name(&self) -> OsString;
    fn extension(&self) -> Option<OsString>;
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
}

impl DirEntry for std::fs::DirEntry {
    fn name(&self) -> OsString {
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

    fn is_dir(&self) -> bool {
        let path = self.path();
        path.is_dir()
    }
}

#[derive(Debug)]
enum Matcher {
    FileExtension(OsString),
    FileName(OsString),
    DirName(OsString),
}

impl Matcher {
    fn matches<E: DirEntry>(&self, entry: &E) -> bool {
        match self {
            Self::FileName(name) => entry.is_file() && &entry.name() == name,
            Self::FileExtension(extension) => {
                entry.is_file() && entry.extension() == Some(extension.clone())
            }
            Self::DirName(name) => entry.is_dir() && &entry.name() == name,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use crate::detector::{Detectors, DirEntry};

    struct FakeDirEntry {
        file_name: OsString,
        extension: Option<OsString>,
        is_file: bool,
        is_dir: bool,
    }

    impl FakeDirEntry {
        fn new<T: Into<OsString>>(
            file_name: T,
            extension: Option<T>,
            is_file: bool,
            is_dir: bool,
        ) -> Self {
            FakeDirEntry {
                file_name: file_name.into(),
                extension: extension.map(|pe| pe.into()),
                is_file,
                is_dir,
            }
        }
    }

    impl DirEntry for FakeDirEntry {
        fn name(&self) -> OsString {
            self.file_name.clone()
        }

        fn extension(&self) -> Option<OsString> {
            self.extension.clone()
        }

        fn is_file(&self) -> bool {
            self.is_file
        }

        fn is_dir(&self) -> bool {
            self.is_dir
        }
    }

    #[test]
    fn detects_java_from_build_gradle() {
        let entry = FakeDirEntry::new("build.gradle", Some("gradle"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert!(result.contains(&"gradle".to_string()));
        assert!(result.contains(&"java".to_string()));
        assert!(result.len() == 2);
    }

    #[test]
    fn detects_java_from_pom_xml() {
        let entry = FakeDirEntry::new("pom.xml", Some("xml"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["java"])
    }

    #[test]
    fn detects_node_from_package_json() {
        let entry = FakeDirEntry::new("package.json", Some("json"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["node"])
    }

    #[test]
    fn detects_python_from_requirements_txt() {
        let entry = FakeDirEntry::new("requirements.txt", Some("txt"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["python"])
    }

    #[test]
    fn detects_haskell_from_dot_cabal() {
        let entry = FakeDirEntry::new("git-ignore.cabal", Some("cabal"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["haskell"])
    }

    #[test]
    fn detects_haskell_from_stack_yaml() {
        let entry = FakeDirEntry::new("stack.yaml", Some("yaml"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["haskell"])
    }

    #[test]
    fn detects_php_from_compose_json() {
        let entry = FakeDirEntry::new("composer.json", Some("json"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["composer"])
    }

    #[test]
    fn detects_ruby_from_gemfile() {
        let entry = FakeDirEntry::new("Gemfile", None, true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["ruby"])
    }

    #[test]
    fn detects_rust_from_cargo_toml() {
        let entry = FakeDirEntry::new("Cargo.toml", Some("toml"), true, false);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["rust"])
    }

    #[test]
    fn detects_scala_from_folder() {
        let entry = FakeDirEntry::new(".metals", None, false, true);
        let result = Detectors::default().detects(&Vec::from([entry]));
        assert_eq!(result, vec!["scala"])
    }
}
