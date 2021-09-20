mod parser;
mod error;

use std::env;
use std::path::Path;
use std::io::Write;

fn main() {
    let mut args = env::args().skip(1);
    
    let inp = match args.next() {
        Some(v) => v,
        None =>  {
            println!("ERROR: Please, provide an input file name");
            std::process::exit(1);
        }
    };
    
    let out = match args.next() {
        Some(v) => v,
        None => { 
            println!("Please, provide an output file name");
            std::process::exit(1);
        }
    };

    let mut inp = match std::fs::File::open(Path::new(&inp)) { 
        Ok(v) => v,
        Err(_) => {
            println!("File with name '{}' not found.", inp);
            std::process::exit(1);
        }
    };

    let mut out = match std::fs::File::create(Path::new(&out)) {
    	Ok(v) => v,
    	Err(_) => {
    		println!("File with name '{}' cannot be created.", out);
    		std::process::exit(1);
    	}
    };

    out.write_all(b"fn main() {\n");
    
    parser::parse_file(&mut inp, &mut out);
}
