use std::io;
use std::io::Write;
use clap::{Parser};

#[derive(Parser)]
#[command(next_line_help = true)]
pub struct Echo {
    /// Write arguments to the standard output
    input: Vec<String>,

    #[arg(short)]
    n: bool
}

impl Echo {
    pub fn run(&self) {
        let output = self.parse_string();
        if self.n {
            print!("{}", output);
            io::stdout().flush().unwrap();
        } else {
            println!("{}", output)
        }
    }

    fn parse_string(&self) -> String {
        let mut new_string = String::new();
        for string in self.input.iter() {
            new_string.push_str(string.as_str());
            new_string.push(' ');
        }
        new_string.pop();
        new_string
    }
}