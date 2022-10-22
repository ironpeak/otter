use error::OtterError;
use regex::Regex;
use std::{process, sync::mpsc, thread};
use walkdir::WalkDir;

use crate::job::Job;

mod config;
mod error;
mod job;
mod language;

fn get_files_in_directory() -> Vec<String> {
    let mut files = Vec::new();
    for file in WalkDir::new(".").into_iter().filter_map(|file| file.ok()) {
        if let Some(path) = file.path().to_str() {
            files.push(path.to_string());
        }
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
        .filter(|x| includes.iter().any(|y| y.is_match(x)))
        .filter(|x| !excludes.iter().any(|y| y.is_match(x)))
        .map(|x| x.to_string())
        .collect())
}

fn get_lang_files(files: &[String], patterns: &Vec<String>) -> Result<Vec<String>, OtterError> {
    let patterns = get_regex(patterns)?;
    Ok(files
        .iter()
        .filter(|x| patterns.iter().any(|y| y.is_match(x)))
        .map(|x| x.to_string())
        .collect())
}

fn otter() -> Result<(), OtterError> {
    let config = config::new();
    println!("config: {}", config);

    let files = get_files(&config.includes, &config.excludes)?;
    println!("files: {}", files.len());
    println!("files: {:?}", files);

    let mut jobs: Vec<Job> = Vec::new();
    for language in language::get_languages() {
        for file in get_lang_files(&files, config.get_files(language))? {
            jobs.push(Job::new(
                language,
                file,
                config.get_flags(language).to_string(),
            )?);
        }
    }

    let mut handles = Vec::new();
    let mut receivers = Vec::new();
    for job in jobs {
        let (sender, receiver) = mpsc::channel();
        receivers.push(receiver);
        handles.push(thread::spawn(move || {
            sender.send(job.run()).unwrap();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = receivers
        .into_iter()
        .map(|x| x.recv().unwrap())
        .collect::<Vec<_>>();
    for result in results.iter().filter(|x| x.success) {
        println!("{}", result);
    }
    for result in results.iter().filter(|x| !x.success) {
        println!("{}", result);
    }

    println!("📖 === Summary === 📖");
    for result in results.iter().filter(|x| x.success) {
        println!("✅ {}", result.command);
    }
    for result in results.iter().filter(|x| !x.success) {
        println!("⛔ {}", result.command);
    }

    if results.iter().any(|x| !x.success) {
        return Err(OtterError::Vulnerability);
    }

    Ok(())
}

fn main() {
    match otter() {
        Ok(_) => {
            println!("🦦 Success 🦦");
        }
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    }
}
