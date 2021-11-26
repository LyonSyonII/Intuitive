mod parser;

use die::die;
use die::Die;
use std::env;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut args = env::args().skip(1);
    if args.len() == 0 {
        die!("Usage: chat INPUT OUTPUT");
    }
    let inp = args
        .next()
        .die("Error: Please, provide a valid input file name");
    let out_s = args
        .next()
        .die("Error: Please, provide an output file name");
    let mut inp =
        std::fs::File::open(Path::new(&inp)).die(&format!("File with name '{}' not found", inp));
    let mut out = std::fs::File::create(Path::new(&(out_s.clone() + ".rs")))
        .die(&format!("File with name '{}' cannot be created", out_s));
    out.write_all(parser::parse_file(&mut inp).as_bytes())
        .die("Could not write to file");
    std::process::Command::new("rustc").args([out_s.clone() + ".rs", "-o".into(), out_s.clone()]).status().die("Looks like you don't have Rust installed, go to https://github.com/LyonSyonII/Intuitive and follow the installation instructions");
    std::process::Command::new(format!("./{}", out_s))
        .status()
        .die("File cannot be executed");
}
