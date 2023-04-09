/*
 * This file is meant to be shared between the main function and the modules made
 */

use std::fs;

use walkdir::WalkDir;
use ansi_term::Color;

pub const VALUE: Color = Color::RGB(0, 175, 255);
pub const WHITE: Color = Color::RGB(255, 255, 255);

/// Get lines of code
pub fn get_loc(files: Vec<String>) -> usize{
    let mut lines = 0;
    for file in files{
        let file = fs::read_to_string(file);
        lines += file.unwrap().split('\n').collect::<Vec<&str>>().len();
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
pub fn format_size(number: usize) -> String{
    let size_names: Vec<String> = vec!["KB", "MB", "GB"].into_iter().map(|e| e.to_string()).collect();
    let mut biggest_name = 0;
    for i in 1..4{
        if number > 10usize.pow(i*3){
            biggest_name = i-1;
        }
    }
    return format!("{:.2}{}", number as f64/10.0f64.powf((biggest_name as f64 + 1.0) * 3.0), size_names[biggest_name as usize]);
}

/// Get the number of characters in all the files
pub fn get_size(files: Vec<String>) -> String{
    let mut size = 0;
    for file in files{
        size +=  fs::read_to_string(file).unwrap().len();
    }
    return format_size(size)
}

/// Get the list of the number of files in certain formats
pub fn get_nb_of_files(files: Vec<String>, formats: Vec<String>) -> Vec<u32>{
    formats.into_iter().map(|format|{
        filter_files_vec_by_format(files.clone(), &format).len() as u32
    }).collect()
}

pub fn get_files(formats: &Vec<String>) -> Vec<String> {
    WalkDir::new(".").into_iter().filter(|project_folder| {
        let name = project_folder.as_ref().unwrap().path().to_str().unwrap();
        for format in formats{
            if name.ends_with(format.as_str()){
                return true;
            }
        }
        false
    }).map(|file| {
        file.unwrap().path().to_str().unwrap().to_string()
    }).collect()
}

pub fn get_formats(args: &Vec<String>) -> Vec<String> {
    args.iter().enumerate().filter(|(index, _)| {
        *index >= 1
    }).map(|(_, format)| {
        format.to_string()
    }).collect()
}

pub fn filter_files_vec_by_format(files: Vec<String>, format: &str) -> Vec<String>{
    files.iter().filter(|file| file.ends_with(format)).map(|file| file.to_string()).collect()
}

pub trait Run{
    fn run(&self);
}

pub mod module_runner{
    use crate::modules::rust_fn_counter::*;
    use crate::modules::rust_dep_counter::*;
    use crate::shared::Run;

    pub fn run_modules(_: Vec<String>, formats: Vec<String>){
        FunctionCounter{formats: formats.clone()}.run();
        DepCounter{formats: formats}.run();
    }
}