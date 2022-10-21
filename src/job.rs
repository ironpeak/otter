use std::{fmt::Display, process::Command};

use crate::language::Language;

pub struct Job {
    directory: Option<String>,
    program: String,
    args: Vec<String>,
}

impl Job {
    pub fn new(language: Language, file: String, flags: String) -> Job {
        match language {
            Language::CSharp => Job {
                directory: Some(file),
                program: "exit".to_string(),
                args: vec!["1".to_string()],
            },
            Language::Go => Job {
                directory: Some(file),
                program: "exit".to_string(),
                args: vec!["1".to_string()],
            },
            Language::JavaScript => Job {
                directory: Some(file),
                program: "exit".to_string(),
                args: vec!["1".to_string()],
            },
            Language::Python => Job {
                directory: Some(file),
                program: "exit".to_string(),
                args: vec!["1".to_string()],
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
        }
    }

    pub fn run(&self) -> JobOutput {
        match Command::new(&self.program).args(&self.args).output() {
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
