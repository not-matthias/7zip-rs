use std::{collections::HashMap, path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};

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
            Ok(output) => output,
        };

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
