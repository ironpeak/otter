use error::OtterError;
use regex::Regex;
use std::process;
use walkdir::WalkDir;

mod config;
mod error;

fn get_files_in_directory() -> Vec<String> {
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

fn get_files(includes: &Vec<String>, excludes: &Vec<String>) -> Result<Vec<String>, OtterError> {
    let includes = get_regex(includes)?;
    let excludes = get_regex(excludes)?;
    Ok(get_files_in_directory()
        .iter()
        .filter(|x| includes.iter().any(|y| y.is_match(&x)))
        .filter(|x| !excludes.iter().any(|y| y.is_match(&x)))
        .map(|x| x.to_string())
        .collect())
}

fn get_lang_files(files: &Vec<String>, patterns: &Vec<String>) -> Result<Vec<String>, OtterError> {
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

    let files = get_files(&config.includes, &config.excludes)?;
    println!("files: {}", files.len());

    let cs_files: Vec<String> = get_lang_files(&files, &config.cs_files)?;
    let go_files: Vec<String> = get_lang_files(&files, &config.go_files)?;
    let js_files: Vec<String> = get_lang_files(&files, &config.js_files)?;
    let py_files: Vec<String> = get_lang_files(&files, &config.py_files)?;
    let rs_files: Vec<String> = get_lang_files(&files, &config.rs_files)?;

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
