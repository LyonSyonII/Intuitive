mod parser;

use std::env;
use std::path::Path;

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
    
    parser::parse_file(&mut inp, out.as_str());
}
