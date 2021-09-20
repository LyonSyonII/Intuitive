use std::io::*;
use std::fs::File;
use std::path::Path;

enum Instruction {
    Assign(String, String),
    Print(String),    
}

pub fn parse_file(file: &mut File, output: &str) {
    // Read file into string
    let mut input = String::new();
    file.read_to_string(&mut input);
    
    for inst in input.split('.') {
        parse_instruction(inst);
    }
}

fn parse_instruction(inst: &str) {
    
    match inst { 
        "print"
    }
}
