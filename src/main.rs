mod module;
mod rust_fn_counter;

use module::*;
use module::modules::*;

use std::{env, fs};


/// Get lines of code
fn get_loc(files: Vec<String>) -> usize{
    let mut lines = 0;
    for file in files{
        let file = fs::read_to_string(file);
        lines += file.unwrap().split('\n').collect::<Vec<&str>>().len();
    }
    lines
}

/// Returns the number with its size name
fn format_size(number: usize) -> String{
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
fn get_size(files: Vec<String>) -> String{
    let mut size = 0;
    for file in files{
        size +=  fs::read_to_string(file).unwrap().len();
    }
    return format_size(size)
}

/// Get the list of the number of files in certain formats
fn get_nb_of_files(files: Vec<String>, formats: Vec<String>) -> Vec<u32>{
    formats.into_iter().map(|format|{
        files.clone().into_iter().filter(|file| file.ends_with(format.as_str())).collect::<Vec<String>>().len() as u32
    }).collect()
}

/// Print the information about the files in the project
fn print_info(files: Vec<String>, formats: Vec<String>){
    let lines = get_loc(files.clone());
    let size = get_size(files.clone());
    let nb_of_files = get_nb_of_files(files.clone(), formats.clone());

    println!("{}", WHITE.bold().paint("[Project Quantifier]"));
    println!("Lines of code: {}", VALUE.paint(format!("{lines}")));
    println!("Code size: {}", VALUE.paint(format!("{size}")));

    println!();

    println!("File number | Size of files | Lines of code");
    println!("-------------------------------------------");

    // Printing all the file formats, the number of files in that format, and the size of the files combined
    formats.iter().enumerate().map(|(index, format)| {
        let current_format_files: Vec<String> = filter_files_vec_by_format(files.clone(), format.as_str());
        let formats_files_size = get_size(current_format_files.clone());
        let format_lines_of_code = get_loc(current_format_files);

        println!("Files {format}: {} {} {}", 
            VALUE.paint(format!("{}", nb_of_files[index])),
            VALUE.paint(format!("[{}]", formats_files_size)),
            VALUE.paint(format!("{}", format_lines_of_code))
        );
    }).for_each(drop);

    println!("-------------------------------------------");
}

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            output_vt100::init();
        }
    }

    let args: Vec<String> = env::args().collect();
    let formats: Vec<String> = get_formats(&args);
    
    let files: Vec<String> = get_files(&formats);

    print_info(files.clone(), formats.clone());
    run_modules(files, formats);
}
