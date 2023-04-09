use std::{env, fs};

use crate::shared::*;
use regex::Regex;

pub struct FunctionCounter{
    pub formats: Vec<String>
}

impl FunctionCounter {
    fn count_captures(&self, re: Regex) -> u32{
        let args: Vec<String> = env::args().collect();
        let formats = get_formats(&args);
        let files = filter_files_vec_by_format(get_files(&formats), ".rs");
    
        let mut counter = 0;
    
        for file in files{
            let file_content = fs::read_to_string(file).unwrap();
    
            counter += re.captures_iter(&file_content).count();
        }

        counter as u32
    }

    fn get_function_counter(&self) -> u32{
        self.count_captures(Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*(->\s*([^{\n;]+))?\s*\{").unwrap())
    }
    
    fn get_struct_counter(&self) -> u32{
        self.count_captures(Regex::new(r"pub\s*struct\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{([^}]*)\}").unwrap())
    }
}

impl Run for FunctionCounter {
    fn run(&self){
        if self.formats.contains(&String::from(".rs")){
            println!("Your Rust code has {} functions and {} structs", 
                VALUE.bold().paint(format!("{}", self.get_function_counter())), 
                VALUE.bold().paint(format!("{}", self.get_struct_counter()))
            );
        }
    }
}