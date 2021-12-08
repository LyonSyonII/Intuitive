pub mod colors;
mod parser;
use colors as color;

use die::die;
use die::Die;
use std::env;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut args = env::args().skip(1);
    if args.len() == 0 {
        die!("Usage: intuitive INPUT OUTPUT DEBUG");
    }

    let inp = args.next().die(&format!(
        "{}ERROR: Please, provide a valid input file name.{}",
        color::RED_BOLD,
        color::DEFAULT
    ));
    let out_s = args.next().die(&format!(
        "{}ERROR: Please, provide an output file name.{}",
        color::RED_BOLD,
        color::DEFAULT
    ));
    let debug = args.next().unwrap_or("".into());
    let debug = debug == "--debug";

    let mut inp =
        std::fs::File::open(Path::new(&inp)).die(&format!("File with name '{}' not found.", inp));
    let mut out = std::fs::File::create(Path::new(&(out_s.clone() + ".rs")))
        .die(&format!("File with name '{}' cannot be created.", out_s));
    out.write_all(parser::parse_file(&mut inp, debug).as_bytes())
        .die("Could not write to file.");
    
    let out = std::process::Command::new("rustc").args([out_s.clone() + ".rs", "-o".into(), out_s.clone() + ".exe"])
        .output()
        .die("Looks like you don't have Rust installed, go to https://github.com/LyonSyonII/Intuitive and follow the installation instructions.");
    std::io::stdout().write_all(&out.stdout).unwrap();
    std::io::stderr().write_all(&out.stderr).unwrap();
    let out = std::process::Command::new(format!("alacritty --{}.exe", out_s))
        .output()
        .unwrap();
    std::io::stdout().write_all(&out.stdout).unwrap();
    std::io::stderr().write_all(&out.stderr).unwrap();
}
