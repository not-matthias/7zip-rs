use std::{collections::HashMap, path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};

pub struct SevenZip {
    pub dir: TempDir,
    pub file_path: PathBuf,
}

impl SevenZip {
    pub fn new(file: &[u8]) -> SevenZip {
        let dir = tempdir().expect("Failed to create temporary directory");

        let file_path = dir.path().join("file.7z");
        std::fs::write(&file_path, file).expect("Failed to write 7zip archive");

        SevenZip { dir, file_path }
    }

    /// Extracts the files and returns them.
    pub fn extract(&self) -> Option<HashMap<String, Vec<u8>>> {
        let executable = std::env::var("7Z_PATH").expect("Couldn't find 7z binary");
        log::trace!("7z binary is at {}", &executable);

        let output = Command::new(executable)
            .arg("x")
            .arg(self.file_path.as_path())
            .arg(format!("-o{}", self.dir.path().display()))
            .output()
            .ok()?;
        if output.status.success() {
            let mut files = HashMap::new();

            for entry in self.dir.path().read_dir().ok()? {
                let path = entry.ok()?.path();

                files.insert(
                    path.file_name()?.to_str()?.to_string(),
                    std::fs::read(path).ok()?,
                );
            }

            Some(files)
        } else {
            log::error!(
                "Extraction failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            None
        }
    }
}
