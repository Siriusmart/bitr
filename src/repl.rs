use std::io::{self, Write};

use crate::*;

pub fn repl() {
    repl_welcome();

    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut lines = vec![String::new()];
    let mut index = 0;

    let mut status = Status::default();

    loop {
        if index == lines.len() {
            print!("> ");
            stdout.flush().expect("cannot write to stdout");

            let mut line = String::new();
            stdin
                .read_line(&mut line)
                .expect("cannot read line from stdin");
            lines.push(line.trim().to_string());
        }

        match lines[index].as_str() {
            "version" => println!("bitr {}", env!("CARGO_PKG_VERSION")),
            "help" => help_msg(),
            _ => {
                if let Err(e) = run_line(&mut lines[index].to_string(), &mut status) {
                    repl_terminated(&e, &lines[index]);
                }
            }
        }

        index += 1;
    }
}
