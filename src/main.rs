use std::{collections::HashMap, env, fs};

use bitr::{repl, run_line, HELP_MSG};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    match args
        .iter()
        .map(|arg| arg.as_str())
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => {
            repl();
            return;
        }
        ["version"] => {
            println!("bitr {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        ["help"] => {
            println!("{HELP_MSG}");
            return;
        }
        _ => {}
    }

    let script = match fs::read_to_string(&args[0]) {
        Ok(s) => s,
        Err(_) => {
            let new_path = format!("{}.bs", args[0]);
            match fs::read_to_string(&new_path) {
                Ok(s) => s,
                Err(_) => {
                    println!("Cannot open file at {} or {new_path}", args[0]);
                    return;
                }
            }
        }
    };

    let mut variables: HashMap<String, Vec<bool>> = HashMap::new();

    for (index, line) in script.lines().enumerate() {
        if let Err(e) = run_line(&mut line.to_string(), &mut variables) {
            println!("---\n\x1b[37mProgram has been terminated with reason: \x1b[91m{e}\x1b[32m\n{} \x1b[0m| \x1b[33m{line}\x1b[0m", index + 1);
            break;
        }
    }
}
