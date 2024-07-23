mod utils;

use utils::*;

use std::env;

/// Print the information about the files in the project
fn print_info(files: Vec<String>, formats: Vec<String>) {
    let lines = get_loc(&files);
    let size = get_size(&files);
    let nb_of_files = get_nb_of_files(&files, &formats);

    println!("{}", WHITE.bold().paint("[Project Quantifier]"));
    println!("Lines of code: {}", VALUE.paint(lines.to_string()));
    println!("Code size: {}", VALUE.paint(size.to_string()));

    println!();

    println!("File number | Size of files | Lines of code");
    println!("-------------------------------------------");

    // Printing all the file formats, the number of files in that format, and the size of the files combined
    formats
        .iter()
        .enumerate()
        .map(|(index, format)| {
            let current_format_files: Vec<String> = filter_files_by_format(&files, format.as_str());
            let formats_files_size = get_size(&current_format_files);
            let format_lines_of_code = get_loc(&current_format_files);

            println!(
                "Files {format}: {} {} {}",
                VALUE.paint(format!("{}", nb_of_files[index])),
                VALUE.paint(format!("[{}]", formats_files_size)),
                VALUE.paint(format!("{}", format_lines_of_code))
            );
        })
        .for_each(drop);

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

    print_info(files, formats);
}
