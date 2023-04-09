# Project Quantifier

This project is a small tool to see the size of your project.

Usage: project_quantifier [file formats you want to look(no binary files like .exe)].

Example:
```
quantifier .rs .md
```
->


![Alt text](assets/screenshot.png)

# Modules

There are currently two modules in this program, [rust functions and structs counter](src/modules/rust_fn_counter.rs) and also [rust dependency counter](src/modules/rust_dep_counter.rs).

You can create your own module to this program by creating a file in [modules](src/modules) and your own struct with the name of your module. After that you'll have to implement the trait Run to implement the run(&self) function. Don't forget to add "pub mod [name of module]"! To access the basic functions, import [shared.rs](src/shared.rs) to get all files and filter by file format for example. The last thing is to run your module in shared::module_runner.