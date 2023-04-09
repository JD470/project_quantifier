use std::process::Command;

use crate::shared::*;

use serde_json::*;
use std::str;

pub struct DepCounter{
    pub formats: Vec<String>
}

impl DepCounter{
    fn get_dependencies(&self, project_path: &str) -> usize {
        let output = Command::new("cargo")
                         .arg("metadata")
                         .arg("--format-version=1")
                         .arg("--manifest-path")
                         .arg(project_path)
                         .output()
                         .unwrap();

        let stdout = str::from_utf8(&output.stdout).unwrap();
        let metadata: Value = serde_json::from_str(stdout).unwrap();

        let packages = metadata["packages"].as_array().unwrap();
        let mut dependencies: Vec<&str> = Vec::new();

        for package in packages {
            let package_dependencies = package["dependencies"].as_array().expect("No dependencies found in package");

            for dep in package_dependencies {
                let name = dep["name"].as_str().unwrap();

                if !dependencies.contains(&name) {
                    dependencies.push(name);
                }
            }
        }
        dependencies.len()
    }
}

impl Run for DepCounter{
    fn run(&self) {
        if self.formats.contains(&String::from(".rs")){
            println!("Your Rust project has {} dependencies",
                VALUE.bold().paint(format!("{}", self.get_dependencies("./Cargo.toml")))
            );
        }
    }
}