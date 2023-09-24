use std::fs;
use std::path::PathBuf;
mod commands;
mod generate_asm;
mod tokenizer;
use clap::{Arg, Command};

fn main() {
    // use clap to parse command line arguments
    let cmd_matches = Command::new("VMTranslator")
    .author("dcbuild3r, dcbuilder@proton.me")
    .version("0.1.0")
    .about("A simple VMTranslator for the Hack computer used in the 1st and 2nd weeks of the Nand2Tetris course part II.")
    .arg(
        Arg::new("input-vm")
        .short('i')
        .long("input")
        .value_name("FILE")
        .help("The input .vm file to assemble.")
        .required(true)
    )
    .get_matches();

    // match the input file
    let vm_file = cmd_matches
        .get_one::<String>("input-vm")
        .expect("required")
        .to_string();

    let mut contents: Vec<String>;
    let mut tokens: Vec<String>;
    let mut asm_code: Vec<String> = Vec::new();
    let mut mult_files = false;
    let mut cur_filename = String::new();

    if vm_file.ends_with(".vm") {
        // if the input file is a single .vm file
        // then we feed the output of the tokenizer into the contents String vector
        contents = match tokenizer::file_contents(vm_file.clone()) {
            Ok(description) => description,
            Err(err) => {
                panic!("There was a problem opening the file: {:?}", err)
            }
        };
        // remove comments
        tokens = tokenizer::remove_comments(contents);
        // generate the assembly code
        // and append to the output string vector
        asm_code.append(&mut generate_asm::asm(
            tokens,
            cur_filename.clone(),
            mult_files,
        ));
        // write the output to a file
        generate_asm::write_output(asm_code, vm_file.clone())
            .map_err(|err| println!("{:?}", err))
            .ok();
    } else {
        mult_files = true;
        let paths = fs::read_dir(vm_file.clone()).unwrap();
        for path in paths {
            cur_filename = path.unwrap().path().display().to_string();
            let path_split = PathBuf::from(cur_filename.clone());
            let crop_filename = path_split
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            contents = match tokenizer::file_contents(cur_filename.clone()) {
                Ok(description) => description,
                Err(err) => {
                    panic!("There was a problem opening the file: {:?}", err)
                }
            };
            tokens = tokenizer::remove_comments(contents);
            asm_code.append(&mut generate_asm::asm(
                tokens,
                crop_filename.clone(),
                mult_files,
            ));
        }
        generate_asm::write_output(asm_code, vm_file.clone())
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
}
