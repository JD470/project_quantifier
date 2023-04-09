use crate::shared::*;


pub struct FunctionCounter{
    pub formats: Vec<String>,
    pub files: Vec<String>,
}

impl FunctionCounter {
    fn get_function_counter(&self) -> u32{
        count_captures(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*(->\s*([^{\n;]+))?\s*\{", &self.files)
    }
    
    fn get_struct_counter(&self) -> u32{
        count_captures(r"pub\s*struct\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\{([^}]*)\}", &self.files)
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