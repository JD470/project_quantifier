pub mod rust_fn_counter;
pub mod rust_dep_counter;

use crate::modules::rust_fn_counter::*;
use crate::modules::rust_dep_counter::*;
use crate::shared::Run;

pub fn run_modules(files: Vec<String>, formats: Vec<String>){
    FunctionCounter{formats: formats.clone(), files: files}.run();
    DepCounter{formats: formats}.run();
}