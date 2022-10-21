use serde::{Deserialize, Serialize};
use std::{env, fmt::Display};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub includes: Vec<String>,
    pub excludes: Vec<String>,

    pub cs_files: Vec<String>,
    pub go_files: Vec<String>,
    pub js_files: Vec<String>,
    pub py_files: Vec<String>,
    pub rs_files: Vec<String>,
}

fn get_config_list(key: &str, default: &str) -> Vec<String> {
    env::var(format!("OTTER_{}", key))
        .unwrap_or(default.to_string())
        .split(",")
        .filter(|&x| !x.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn new() -> Config {
    Config {
        includes: get_config_list("INCLUDES", "."),
        excludes: get_config_list("EXCLUDES", ""),

        cs_files: get_config_list("CS_FILES", "package-lock.json"),
        go_files: get_config_list("GO_FILES", "go.mod,go.sum"),
        js_files: get_config_list("JS_FILES", "package-lock.json,yarn.lock"),
        py_files: get_config_list("PY_FILES", "requirements.txt"),
        rs_files: get_config_list("RS_FILES", "Cargo.lock"),
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
