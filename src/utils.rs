use std::{
    fs::{self, File},
    path::PathBuf,
};

use ansi_term::Color;
use jwalk::WalkDir;

use crate::languages::Languages;

pub const VALUE: Color = Color::RGB(0, 175, 255);
pub const WHITE: Color = Color::RGB(255, 255, 255);

fn count_lines(text: String) -> usize {
    let mut lines: usize = 0;
    let mut multiline_comment = false;

    for line in text.lines().map(|line| line.trim()) {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("//") || line.starts_with('#') {
            continue;
        }

        if multiline_comment {
            if contains_string(line, "*/") {
                multiline_comment = false;
            }
            continue;
        }

        if contains_string(line, "/*") && !multiline_comment {
            multiline_comment = true;
            continue;
        }

        lines += 1;
    }

    lines
}

fn contains_string(text: &str, needle: &str) -> bool {
    let mut start = 0;
    let mut in_string = false;

    for (i, c) in text.chars().enumerate() {
        if !(c == '\"' || c == '\'') && in_string {
            continue;
        }

        in_string = !in_string;
        start = i;

        if let Some(slice) = text.get(start..i) {
            if slice.contains(needle) {
                return true;
            }
        }
    }

    match text.get(start..text.len()) {
        Some(slice) => slice.contains(needle),
        _ => false,
    }
}

/// Get lines of code
pub fn get_loc(files: &[String]) -> usize {
    let mut lines: usize = 0;
    for file in files {
        if let Ok(file) = fs::read_to_string(file) {
            lines += count_lines(file);
        }
    }
    lines
}

/// Returns the number with its size name
///
/// Example:
///
/// 1 024 = 1.00KB
///
/// 1 024 000 = 1.00MB
///
/// 1 024 000 000 = 1.00GB
///
pub fn format_size_bytes(number: usize) -> String {
    let size_names: Vec<&str> = vec!["B", "KB", "MB", "GB"];
    let biggest_name = (number as f64).log(1024f64).floor() as usize;

    if biggest_name == 0 {
        format!("{}{}", number, size_names[biggest_name])
    } else {
        format!(
            "{:.2}{}",
            (number as f64) / 1024f64.powf(biggest_name as f64),
            size_names[biggest_name]
        )
    }
}

/// Get the number of characters in all the files
pub fn get_size(files: &[String]) -> String {
    let mut size = 0;
    unsafe {
        for file in files {
            size += File::open(file)
                .unwrap_unchecked()
                .metadata()
                .unwrap_unchecked()
                .len() as usize;
        }
    }

    format_size_bytes(size)
}

pub fn get_files(formats: &[String]) -> Vec<String> {
    let languages: Vec<Languages> = formats.iter().map(|a| Languages::from(a)).collect();

    let mut first_depth_files: Vec<PathBuf> = WalkDir::new(".")
        .parallelism(jwalk::Parallelism::Serial)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type.is_file())
        .map(|file| file.path())
        .collect();

    let first_depth_folders: Vec<PathBuf> = WalkDir::new(".")
        .parallelism(jwalk::Parallelism::Serial)
        .into_iter()
        .skip(1) // Skip root folder
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type.is_dir())
        .filter(|folder| {
            languages
                .iter()
                .any(|language| language.exclude_directory(folder.path()))
        })
        .map(|folder| folder.path())
        .collect();

    first_depth_files.extend(first_depth_folders.iter().flat_map(|folder| {
        WalkDir::new(folder)
            .parallelism(jwalk::Parallelism::Serial)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type.is_file())
            .map(|file| file.path())
            .collect::<Vec<PathBuf>>()
    }));

    unsafe {
        first_depth_files
            .iter()
            .map(|file| file.to_str().unwrap_unchecked().to_string())
            .filter(|file| formats.iter().any(|format| file.ends_with(format)))
            .collect()
    }
}

pub fn get_formats(args: &[String]) -> Vec<String> {
    args.iter()
        .skip(1)
        .map(|format| format.to_owned())
        .collect()
}

pub fn filter_files_by_format(files: &[String], format: &str) -> Vec<String> {
    files
        .iter()
        .filter(|file| file.ends_with(format))
        .map(|file| file.to_string())
        .collect()
}
