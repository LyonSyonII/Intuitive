use std::io::*;
use std::fs::File;
use std::path::Path;

enum Instruction {
    Assign(String, String),
    Print(String),    
}

pub fn parse_file(file: &mut File, out: &mut File) {
    // Read file into string
    let mut inp = String::new();
    file.read_to_string(&mut inp);

    // Read instructions
    for expr in inp.split('.') {
        out.write_all(parse_expression(expr).as_bytes());
    }

    out.write_all(b"\n}");
}

fn parse_expression(expr: &str) -> String {
	let mut out = String::new();

	// Get individual words
	let mut expr = expr.split(' ');
	// TODO: Filter linker words 
    for word in expr {
        match word { 
            "print" => {},
            _ => {}
        }
    }
    
    out
}
