use atty::Stream;
use std::io::{self, Read};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    formatter: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    if atty::is(Stream::Stdin) {
        return;
    }

    let formatter = match args.formatter {
        Some(f) => f,
        None => "".to_string(),
    };

    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer).unwrap();
    let mut it = buffer.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '\n' => print!("{}", formatter),
            '\r' => {
                print!("{}", formatter);
                // skip \n after \r and only print formatter once for the \r in \r\n
                if let Some(c) = it.peek() {
                    if *c == '\n' {
                        it.next();
                    }
                }
            }
            _ => print!("{}", c),
        };
    }

    println!();
}
