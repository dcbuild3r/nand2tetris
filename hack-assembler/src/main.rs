use clap::{Arg, Command};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub use assembler::Assembler;

mod assembler;

fn main() {
    let cmd_matches = Command::new("HackAssembler")
    .author("dcbuild3r, dcbuilder@proton.me")
    .version("0.1.0")
    .about("A simple assembler for the Hack computer used in the 6th week of the Nand2Tetris course part I.")
    .arg(
        Arg::new("input-asm")
        .short('i')
        .long("input")
        .value_name("FILE")
        .help("The input .asm file to assemble.")
        .required(true)
    )
    .get_matches();

    let input_asm = cmd_matches
        .get_one::<String>("input-asm")
        .expect("required")
        .to_string();

    let mut contents = String::new();

    let mut file = File::open(&input_asm).expect("File not found");

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let output_hack = str::replace(&input_asm, ".asm", ".hack");
    let mut assembler = Assembler::new(&contents);
    let hack = assembler.assemble();

    let path = Path::new(&output_hack);
    let display = path.display();

    let mut file = File::create(&path).expect(&format!("Couldn't create {}", display));

    file.write_all(hack.as_bytes())
        .expect(&format!("Couldn't write to {}", display));
}
