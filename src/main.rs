use error::OtterError;
use regex::Regex;
use std::process;
use walkdir::WalkDir;

mod config;
mod error;

fn get_files() -> Vec<String> {
    let mut files = Vec::new();
    for file in WalkDir::new(".").into_iter().filter_map(|file| file.ok()) {
        match file.path().to_str() {
            Some(path) => files.push(path.replace("\\", "/")),
            None => {}
        };
    }
    files
}

pub fn get_regex(patterns: &Vec<String>) -> Result<Vec<Regex>, OtterError> {
    let mut includes = Vec::new();
    for pattern in patterns {
        match Regex::new(pattern) {
            Ok(regex) => includes.push(regex),
            Err(_) => {
                return Err(OtterError::RegexError {
                    pattern: pattern.to_string(),
                })
            }
        }
    }
    Ok(includes)
}

fn find_matches(files: &Vec<String>, patterns: &Vec<String>) -> Result<Vec<String>, OtterError> {
    let patterns = get_regex(patterns)?;
    Ok(files
        .iter()
        .filter(|x| patterns.iter().any(|y| y.is_match(&x)))
        .map(|x| x.to_string())
        .collect())
}

fn otter() -> Result<(), OtterError> {
    let config = config::new();
    println!("config: {}", config);

    let include_patterns = get_regex(&config.includes)?;
    let exclude_patterns = get_regex(&config.excludes)?;
    let files: Vec<String> = get_files()
        .into_iter()
        .filter(|x| !exclude_patterns.iter().any(|y| y.is_match(&x)))
        .filter(|x| include_patterns.iter().any(|y| y.is_match(&x)))
        .collect();

    println!("files: {}", files.len());

    let cs_files: Vec<String> = find_matches(&files, &config.cs_files)?;
    println!(
        "cs_files: {}",
        serde_json::to_string_pretty(&cs_files).unwrap()
    );

    let go_files: Vec<String> = find_matches(&files, &config.go_files)?;
    println!(
        "go_files: {}",
        serde_json::to_string_pretty(&go_files).unwrap()
    );

    let js_files: Vec<String> = find_matches(&files, &config.js_files)?;
    println!(
        "js_files: {}",
        serde_json::to_string_pretty(&js_files).unwrap()
    );

    let py_files: Vec<String> = find_matches(&files, &config.py_files)?;
    println!(
        "py_files: {}",
        serde_json::to_string_pretty(&py_files).unwrap()
    );

    let rs_files: Vec<String> = find_matches(&files, &config.rs_files)?;
    println!(
        "rs_files: {}",
        serde_json::to_string_pretty(&rs_files).unwrap()
    );

    Ok(())
}

fn main() {
    match otter() {
        Ok(_) => {
            println!("Done!");
        }
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    }
}
