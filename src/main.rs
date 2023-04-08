use std::{env, fs};
use ansi_term::Color;
use walkdir::WalkDir;

const VALUE: Color = Color::RGB(0, 175, 255);
const IN_PARENTHESIS: Color = Color::RGB(150, 150, 150);
const WHITE: Color = Color::RGB(255, 255, 255);

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

    // Printing all the file formats, the number of files in that format, and the size of the files combined
    formats.iter().enumerate().map(|(index, format)| {
        let formats_files_size = get_size(files.iter().filter(|file| file.ends_with(format)).map(|f| f.to_string()).collect::<Vec<String>>());
        println!("Files {format}: {} {}", 
            VALUE.paint(format!("{}", nb_of_files[index])),
            IN_PARENTHESIS.paint(format!("[{}]", formats_files_size))
        );
    }).for_each(drop);
}

fn main() {
    cfg_if::cfg_if! {
		if #[cfg(target_os = "windows")] {
			output_vt100::init();
		}
	}

    let args: Vec<String> = env::args().collect();
    let formats: Vec<String> = args.iter().enumerate().filter(|(index, _)| {
        *index >= 1
    }).map(|(_, format)| {
        format.to_string()
    }).collect();
    
    let files: Vec<String> = WalkDir::new(".").into_iter().filter(|f| {
        let name = f.as_ref().unwrap().path().to_str().unwrap();
        for format in &formats{
            if name.ends_with(format.as_str()){
                return true;
            }
        }
        false
    }).map(|file| {
        file.unwrap().path().to_str().unwrap().to_string()
    }).collect();

    print_info(files, formats);
}
