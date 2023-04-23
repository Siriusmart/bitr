use std::env;

use bitr::*;

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
            help_msg();
            return;
        }
        _ => {}
    }

    let (status, lines) = match Status::try_from_path(&args[0], None) {
        Some(status) => status,
        None => return,
    };

    if run_script(status, lines).is_none() {
        terminated_due_to_above_errors();
    }
}
