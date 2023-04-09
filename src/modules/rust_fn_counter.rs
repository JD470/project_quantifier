use crate::shared::*;
use regex::Regex;

pub struct FunctionCounter{
    pub formats: Vec<String>,
    pub files: Vec<String>,
}

impl FunctionCounter {
    fn get_function_counter(&self) -> u32{
        count_captures(Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*(->\s*([^{\n;]+))?\s*\{").unwrap(), &self.files)
    }
    
    fn get_struct_counter(&self) -> u32{
        count_captures(Regex::new(r"pub\s*struct\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{([^}]*)\}").unwrap(), &self.files)
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