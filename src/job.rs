use std::{fmt::Display, path::PathBuf, process::Command};

use crate::{error::OtterError, language::Language};

pub struct Job {
    directory: Option<String>,
    program: String,
    args: Vec<String>,
}

fn get_filename(path: &str) -> Option<String> {
    let path = PathBuf::from(path);
    let filename = path.file_name();
    match filename {
        Some(filename) => filename.to_str().map(|x| x.to_string()),
        None => None,
    }
}

fn get_directory(path: &str) -> Option<String> {
    let path = PathBuf::from(path);
    let dir = path.parent();
    match dir {
        Some(dir) => dir.to_str().map(|x| x.to_string()),
        None => None,
    }
}

impl Job {
    pub fn new(language: Language, file: String, flags: String) -> Result<Job, OtterError> {
        Ok(match language {
            Language::CSharp => Job {
                directory: get_directory(&file),
                program: "dotnet".to_string(),
                args: format!("list package --vulnerable {}", flags)
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            },
            Language::Go => Job {
                directory: get_directory(&file),
                program: "govulncheck".to_string(),
                args: vec!["./...".to_string()],
            },
            Language::JavaScript => match get_filename(&file).as_deref() {
                Some("yarn.lock") => Job {
                    directory: get_directory(&file),
                    program: "yarn".to_string(),
                    args: vec!["audit".to_string()],
                },
                Some(_) => Job {
                    directory: get_directory(&file),
                    program: "npm".to_string(),
                    args: vec!["audit".to_string()],
                },
                None => return Err(OtterError::UnknownFile { file }),
            },
            Language::Python => Job {
                directory: None,
                program: "pip-audit".to_string(),
                args: format!("--requirement {} {}", file, flags)
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            },
            Language::Rust => Job {
                directory: None,
                program: "cargo-audit".to_string(),
                args: format!("audit --file {} {}", file, flags)
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            },
        })
    }

    pub fn run(&self) -> JobOutput {
        let mut cmd = Command::new(&self.program);
        if let Some(directory) = &self.directory {
            cmd.current_dir(directory);
        }
        match cmd.args(&self.args).output() {
            Ok(output) => JobOutput {
                command: format!("{} {}", self.program, self.args.join(" ")),
                output: String::from_utf8_lossy(&output.stdout).to_string(),
                success: output.status.success(),
            },
            Err(err) => JobOutput {
                command: format!("{} {}", self.program, self.args.join(" ")),
                output: format!("Error: could not spawn child process {}", err),
                success: false,
            },
        }
    }
}

pub struct JobOutput {
    pub command: String,
    pub output: String,
    pub success: bool,
}

impl Display for JobOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.output)?;
        write!(
            f,
            "{}",
            match self.success {
                true => "✅",
                false => "⛔",
            }
        )?;
        write!(f, " {}", self.command)?;
        Ok(())
    }
}
