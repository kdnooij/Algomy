use std::{fs, io::Write};

use parser::parse;

use crate::simplify::simplify;
use clap::Parser as ClapParser;

mod evaluate;
mod expression;
mod parser;
mod simplify;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.input {
        let program = fs::read_to_string(path).expect("Unable to read file");

        match parse(program.as_str()) {
            Ok(lines) => {
                for line in lines {
                    println!("{}", simplify(&line));
                }
            }
            Err(err) => {
                eprintln!("Interpreter Error:");
                eprint!("{}", err.to_string())
            }
        }
    } else {
        // REPL mode
        loop {
            let mut input = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == ":exit" {
                break;
            }

            match parse(input.as_str()) {
                Ok(lines) => {
                    for line in lines {
                        println!("{}", simplify(&line));
                    }
                }
                Err(err) => {
                    eprintln!("Interpreter Error:");
                    eprint!("{}", err.to_string())
                }
            }
        }
    }
}
