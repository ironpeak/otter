use serde::{Deserialize, Serialize};
use std::{env, fmt::Display};

use crate::language::Language;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub includes: Vec<String>,
    pub excludes: Vec<String>,

    pub cs_files: Vec<String>,
    pub go_files: Vec<String>,
    pub js_files: Vec<String>,
    pub py_files: Vec<String>,
    pub rs_files: Vec<String>,

    pub cs_flags: String,
    pub go_flags: String,
    pub js_flags: String,
    pub py_flags: String,
    pub rs_flags: String,
}

fn get_config(key: &str, default: &str) -> String {
    env::var(format!("OTTER_{}", key)).unwrap_or_else(|_| default.to_string())
}

fn get_config_list(key: &str, default: &str) -> Vec<String> {
    get_config(key, default)
        .split(',')
        .filter(|&x| !x.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn new() -> Config {
    Config {
        includes: get_config_list("INCLUDES", "."),
        excludes: get_config_list("EXCLUDES", ""),

        cs_files: get_config_list("CS_FILES", "/packages.lock.json$"),
        go_files: get_config_list("GO_FILES", "/go.mod$,/go.sum$"),
        js_files: get_config_list("JS_FILES", "/package-lock.json$,/yarn.lock$"),
        py_files: get_config_list("PY_FILES", "/requirements.txt$"),
        rs_files: get_config_list("RS_FILES", "/Cargo.lock$"),

        cs_flags: get_config("CS_FLAGS", "--include-transitive"),
        go_flags: get_config("GO_FLAGS", ""),
        js_flags: get_config("JS_FLAGS", "--frozen-lockfile --production"),
        py_flags: get_config("PY_FLAGS", ""),
        rs_flags: get_config("RS_FLAGS", ""),
    }
}

impl Config {
    pub fn get_files(&self, language: Language) -> &Vec<String> {
        match language {
            Language::CSharp => &self.cs_files,
            Language::Go => &self.go_files,
            Language::JavaScript => &self.js_files,
            Language::Python => &self.py_files,
            Language::Rust => &self.rs_files,
        }
    }

    pub fn get_flags(&self, language: Language) -> &str {
        match language {
            Language::CSharp => &self.cs_flags,
            Language::Go => &self.go_flags,
            Language::JavaScript => &self.js_flags,
            Language::Python => &self.py_flags,
            Language::Rust => &self.rs_flags,
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
