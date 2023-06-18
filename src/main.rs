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
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "q" => break,
            "p" => editor::print_lines(&lines),
            "a" => editor::append_line(&mut lines),
            "d" => editor::delete_line(&mut lines),
            _ => println!("Invalid command"),

        }
    }

    if let Err(err) = fs::write(filename, lines.join("\n")) {
        eprintln!("Failed to save file: {}", err);
    }
}
