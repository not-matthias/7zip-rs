use std::{collections::HashMap, path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;

pub struct SevenZip {
    pub dir: TempDir,
    pub file_path: PathBuf,
}

impl SevenZip {
    pub fn new_file(file_path: PathBuf) -> Option<SevenZip> {
        let content = std::fs::read(&file_path).ok()?;
        Some(Self::new(file_path.file_name()?.to_str()?, &content))
    }

    pub fn new(name: &str, file: &[u8]) -> SevenZip {
        let dir = tempdir().expect("Failed to create temporary directory");

        let file_path = dir.path().join(name);
        std::fs::write(&file_path, file).expect("Failed to write 7zip archive");

        SevenZip { dir, file_path }
    }

    /// Extracts the files and returns them.
    pub fn extract(&self) -> Option<HashMap<String, Vec<u8>>> {
        let executable = if cfg!(target_os = "windows") {
            std::env::temp_dir().join("7zr.exe")
        } else {
            "7z".into()
        };

        log::debug!(
            "Extracting {} to {}",
            self.file_path.display(),
            self.dir.path().display()
        );
        let output = Command::new(executable)
            .arg("x")
            .arg(self.file_path.as_path())
            .arg(format!("-o{}", self.dir.path().display()))
            .output();

        let output = match &output {
            Err(error) => {
                log::error!("Failed to run command: {:?}", error);
                return None;
            }
            Ok(output) => {
                log::debug!("Successfully ran the command. {}", output.status);
                output
            }
        };

        if output.status.success() {
            let mut files = HashMap::new();

            for entry in WalkDir::new(self.dir.path())
                .into_iter()
                .filter_map(|e| e.ok())
            {
                log::debug!("Found entry: {}", entry.path().display());

                let name = entry.file_name().to_string_lossy();
                let Ok(content) = std::fs::read(entry.path()) else {
                    continue;
                };

                files.insert(name.to_string(), content);
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
