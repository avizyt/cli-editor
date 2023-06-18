use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

pub fn print_lines(lines: &[String]) {
    for line in lines {
        println!("{}", line);
    }
}

pub fn append_line(lines: &mut Vec<String>){
    print!("Enter a new line: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    lines.push(input.trim().to_owned());
}

pub fn delete_line(lines: &mut vec<String>) {
    print!("Enter the line number to delete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(index) = input.trim().parse::<usize> {
        if index > 0 && index <= lines.len() {
            lines.remove(index - 1);
        } else {
            println!("Invalid line number");
        }
    } else {
        println!("Invalid input");
    }
}