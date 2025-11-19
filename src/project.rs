use anyhow::{Result, bail};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a Noir project on disk.
#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub manifest_path: PathBuf,
}

impl Project {
    /// Construct a Project from a given root directory.
    /// Checks that:
    ///   - root is a directory
    ///   - Nargo.toml exists in the root
    pub fn from_root(root: PathBuf) -> Result<Self> {
        let root = root.canonicalize()?;

        if !root.is_dir() {
            bail!("Project root {} is not a directory", root.display());
        }

        let manifest_path = root.join("Nargo.toml");

        if !manifest_path.is_file() {
            bail!("No Nargo.toml found in project root {}", root.display());
        }

        Ok(Project {
            root,
            manifest_path,
        })
    }

    /// Find all `.nr` files under the project root (recursively).
    pub fn nr_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if path.is_file() && is_nr_file(path) {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }
}

fn is_nr_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "nr")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_nr_files_in_fixture() {
        let root = PathBuf::from("tests/fixtures/simple_noir");
        let project = Project::from_root(root).expect("project should be valid");

        let files = project.nr_files().expect("nr_files should succeed");

        let joined_paths: Vec<String> = files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        assert!(
            joined_paths.iter().any(|p| p.ends_with("src/main.nr")),
            "expected to find src/main.nr in nr_files, got: {:?}",
            joined_paths,
        );
    }
}
