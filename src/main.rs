use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod editor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let mut lines: Vec<String> = if let Ok(contents) = fs::read_to_string(filename) {
        contents.lines().map(|s| s.to_owned()).collect()
    } else {
        Vec::new()
    };

    loop {
        
    }
}
