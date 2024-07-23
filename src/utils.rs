use std::fs;

use ansi_term::Color;
use walkdir::WalkDir;

pub const VALUE: Color = Color::RGB(0, 175, 255);
pub const WHITE: Color = Color::RGB(255, 255, 255);

fn count_lines(text: String) -> usize {
    let mut lines: usize = 0;
    let mut multiline_comment = false;

    for line in text.lines().map(|line| line.trim()) {
        if multiline_comment && !contains_string(line, "*/") {
            continue;
        } else if multiline_comment && contains_string(line, "*/") {
            multiline_comment = false
        }

        if line.is_empty() {
            continue;
        }
        if line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        if contains_string(line, "/*") && !multiline_comment {
            multiline_comment = true;
            continue;
        }

        if !multiline_comment {
            lines += 1;
        }
    }

    lines
}

fn contains_string(text: &str, needle: &str) -> bool {
    let mut start = 0;
    let mut in_string = false;
    let mut parts: Vec<String> = Vec::new();

    for (i, c) in text.chars().enumerate() {
        if in_string && (c == '\"' || c == '\'') {
            in_string = !in_string;
            start = i;
            continue;
        }
        if !in_string && (c == '\"' || c == '\'') {
            in_string = !in_string;
            start = i;
            parts.push(text[start..i].to_string());
            continue;
        }
    }

    parts.push(text[start..text.len()].to_string());
    parts.concat().contains(needle)
}

/// Get lines of code
pub fn get_loc(files: &Vec<String>) -> usize {
    let mut lines: usize = 0;
    for file in files {
        let file = fs::read_to_string(file);
        lines += count_lines(file.unwrap());
    }
    lines
}

/// Returns the number with its size name
///
/// Example:
///
/// 1 000 = 1.00KB
///
/// 1 000 000 = 1.00MB
///
/// 1 000 000 000 = 1.00GB
///
pub fn format_size_bytes(number: usize) -> String {
    let size_names: Vec<String> = vec!["KB", "MB", "GB"]
        .into_iter()
        .map(|e| e.to_string())
        .collect();
    let mut biggest_name = 0;
    for i in 1..4 {
        if number > 10usize.pow(i * 3) {
            biggest_name = i - 1;
        }
    }
    
    format!(
        "{:.2}{}",
        number as f64 / 10.0f64.powf((biggest_name as f64 + 1.0) * 3.0),
        size_names[biggest_name as usize]
    )
}

/// Get the number of characters in all the files
pub fn get_size(files: &[String]) -> String {
    let mut size = 0;
    for file in files {
        size += fs::read_to_string(file).unwrap().len();
    }
    
    format_size_bytes(size)
}

/// Get the list of the number of files in certain formats
pub fn get_nb_of_files(files: &[String], formats: &[String]) -> Vec<u32> {
    formats
        .iter()
        .map(|format| filter_files_by_format(files, format).len() as u32)
        .collect()
}

pub fn get_files(formats: &[String]) -> Vec<String> {
    WalkDir::new(".")
        .into_iter()
        .filter(|project_folder| {
            let name = project_folder.as_ref().unwrap().path().to_str().unwrap();

            formats.iter().any(|format| name.ends_with(format))
        })
        .map(|file| file.unwrap().path().to_str().unwrap().to_string())
        .collect()
}

pub fn get_formats(args: &[String]) -> Vec<String> {
    args.iter()
        .enumerate()
        .filter(|(index, _)| *index >= 1)
        .map(|(_, format)| format.to_string())
        .collect()
}

pub fn filter_files_by_format(files: &[String], format: &str) -> Vec<String> {
    files
        .iter()
        .filter(|file| file.ends_with(format))
        .map(|file| file.to_string())
        .collect()
}
