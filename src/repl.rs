use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{run_line, HELP_MSG};

pub fn repl() {
    println!(
        "Welcome to bitr {}.\nType \"help\" for more information.",
        env!("CARGO_PKG_VERSION")
    );

    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut lines = Vec::new();
    let mut variables: HashMap<String, Vec<bool>> = HashMap::new();

    loop {
        print!("> ");
        stdout.flush().expect("cannot write to stdout");

        let mut line = String::new();
        stdin
            .read_line(&mut line)
            .expect("cannot read line from stdin");

        match line.trim() {
            "version" => println!("bitr {}", env!("CARGO_PKG_VERSION")),
            "help" => println!("{HELP_MSG}"),
            _ => match run_line(&mut line.clone(), &mut variables) {
                Ok(()) => lines.push(line),
                Err(e) => {
                    println!("---\n\x1b[37mFailed with reason: \x1b[91m{e}\n\x1b[33m{line}\x1b[0m");
                }
            },
        }
    }
}
