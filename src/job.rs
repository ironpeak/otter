use std::{fmt::Display, path::PathBuf};

use regex::Regex;
use subprocess::Exec;

use crate::{error::OtterError, language::Language};

#[derive(Eq, Hash, PartialEq)]
pub struct Job {
    command: String,
    failure_pattern: Option<String>,
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
                command: format!(
                    "cd {} && dotnet restore --locked-mode && dotnet list package --vulnerable {}",
                    get_directory(&file).as_deref().unwrap_or("."),
                    flags
                ),
                failure_pattern: Some("has the following vulnerable packages".to_string()),
            },
            Language::Go => Job {
                command: format!(
                    "cd {} && go list -json -m all | nancy sleuth {}",
                    get_directory(&file).as_deref().unwrap_or("."),
                    flags
                ),
                failure_pattern: None,
            },
            Language::JavaScript => match get_filename(&file).as_deref() {
                Some("yarn.lock") => Job {
                    command: format!(
                        "cd {} && yarn audit {}",
                        get_directory(&file).as_deref().unwrap_or("."),
                        flags
                    ),
                    failure_pattern: None,
                },
                Some(_) => Job {
                    command: format!(
                        "cd {} && npm audit {}",
                        get_directory(&file).as_deref().unwrap_or("."),
                        flags
                    ),
                    failure_pattern: None,
                },
                None => return Err(OtterError::UnknownFile { file }),
            },
            Language::Python => Job {
                command: format!("pip-audit --requirement {} {}", file, flags),
                failure_pattern: None,
            },
            Language::Rust => Job {
                command: format!("cargo-audit audit --file {} {}", file, flags),
                failure_pattern: None,
            },
        })
    }

    pub fn run(&self) -> JobOutput {
        match Exec::shell(&self.command).capture() {
            Ok(data) => {
                let output = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&data.stdout),
                    String::from_utf8_lossy(&data.stderr)
                );
                JobOutput {
                    command: self.command.clone(),
                    output: output.clone(),
                    success: match self.failure_pattern {
                        Some(ref pattern) => !Regex::new(pattern).unwrap().is_match(&output),
                        None => data.exit_status.success(),
                    },
                }
            }
            Err(err) => JobOutput {
                command: self.command.clone(),
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
        write!(
            f,
            "{}",
            match self.success {
                true => "✅",
                false => "⛔",
            }
        )?;
        write!(f, " {}", self.command)?;
        writeln!(f, "{}", self.output)?;
        Ok(())
    }
}
