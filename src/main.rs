mod parser;
mod db;

use std::env;
use std::path::Path;
use std::io::Write;
use die::Die;

fn main() {
    let mut args = env::args().skip(1);
    if args.len() == 0 { die!("Usage: chat INPUT OUTPUT"); }
    let inp = args.next().die("Error: Please, provide a valid input file name");
    let out = args.next().die("Error: Pleas, provide an output file name");

    let mut inp = std::fs::File::open(Path::new(&inp)).die(&format!("File with name '{}' not found", inp));
    let mut out = std::fs::File::create(Path::new(&out)).die(&format!("File with name '{}' cannot be created.", out));

    out.write_all(b"fn main() {\n");
    parser::parse_file(&mut inp, &mut out);
}
