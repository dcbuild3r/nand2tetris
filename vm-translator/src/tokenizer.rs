use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// read file contents into a buffer and return a vector of strings
pub fn file_contents(filename: String) -> io::Result<Vec<String>> {
    let mut contents = Vec::new();
    let file = File::open(filename).expect("Something went wrong reading the file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        contents.push(line?);
    }

    Ok(contents)
}

/// remove comments and empty lines from the contents vector
pub fn remove_comments(contents: Vec<String>) -> Vec<String> {
    let mut tokens = Vec::new();
    for mut line in contents {
        if line.find('/') != None {
            let idx = line.find("//").unwrap_or(line.len());
            line.replace_range(idx.., "");
        }
        if line.is_empty() == false {
            let mut prev = ' ';
            line.retain(|ch| {
                let result = ch != ' ' || prev != ' ';
                prev = ch;
                result
            });
            tokens.push(line.trim().to_string());
        }
    }
    tokens
}
