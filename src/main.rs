use std::{fs, io::Write};

use parser::AlgomyKernel;

use crate::{parser::Line, simplify::simplify};
use clap::Parser as ClapParser;

mod classify;
mod evaluate;
mod expression;
mod kernel;
mod parser;
mod polynomial;
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

    let mut kernel = AlgomyKernel::new();

    if let Some(path) = args.input {
        let program = fs::read_to_string(path).expect("Unable to read file");

        match kernel.parse_program(program.as_str()) {
            Ok(lines) => {
                for line in lines {
                    if let Some(result) = kernel.evaluate_line(line) {
                        println!("{}", result);
                    }
                }
            }
            Err(err) => {
                eprintln!("Interpreter Error:");
                eprintln!("{}", err.to_string())
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

            let args = input.trim().split(" ").collect::<Vec<_>>();
            match args[..] {
                [":Exit"] => break,
                [":ClearSession"] => kernel.clear_session(),
                [":Clear", var] => {
                    if let Line::Expr(var) = kernel.parse_line(var).unwrap() {
                        kernel.clear_variable(&var);
                    }
                }
                _ => match kernel.parse_line(input.as_str()) {
                    Ok(line) => {
                        if let Some(result) = kernel.evaluate_line(line) {
                            println!("{}", result);
                        }
                    }
                    Err(err) => {
                        eprintln!("Interpreter Error:");
                        eprintln!("{}", err.to_string())
                    }
                },
            }
        }
    }
}
