use walkdir::WalkDir;
use ansi_term::Color;

pub const VALUE: Color = Color::RGB(0, 175, 255);
pub const WHITE: Color = Color::RGB(255, 255, 255);

pub fn get_files(formats: &Vec<String>) -> Vec<String> {
    WalkDir::new(".").into_iter().filter(|f| {
        let name = f.as_ref().unwrap().path().to_str().unwrap();
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

pub mod modules{
    use crate::rust_fn_counter::*;
    use crate::module::Run;

    pub fn run_modules(_: Vec<String>, formats: Vec<String>){
        FunctionCounter{formats: formats}.run();
    }
}