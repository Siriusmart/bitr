pub fn file_terminated(e: &str, line_no: usize, line: &str) {
    println!("---\n\x1b[37mProgram has been terminated with reason: \x1b[91m{e}\x1b[32m\n{line_no} \x1b[0m| \x1b[33m{line}\x1b[0m");
}

pub fn repl_terminated(e: &str, line: &str) {
    println!("---\n\x1b[37mFailed with reason: \x1b[91m{e}\n\x1b[33m{line}\x1b[0m");
}

pub fn multiple_definitions_lbl(lbl: &str, original: usize) -> String {
    format!("Multiple definition of label {lbl} - it is also defined in line {original}")
}

pub fn repl_welcome() {
    println!(
        "Welcome to bitr {}.\nType \"help\" for more information.",
        env!("CARGO_PKG_VERSION")
    );
}

pub fn help_msg() {
    println!(
        "\x1b[32mBitr - a BitScript interpreter\x1b[0m

\x1b[37mFor more about BitScript and Bitr, visit https://siriusmart.github.io/bitscript\x1b[0m

\x1b[91mCOMMANDS:\x1b[0m
    \x1b[33mbitr\x1b[0m                 Starts the BitScript repl
    \x1b[33mbitr help\x1b[0m            Shows this message
    \x1b[33mbitr version\x1b[0m         Displays Bitr version
    \x1b[33mbitr [file name]\x1b[0m     Run from a file

\x1b[37mFile extension is not required if it is `.bs`.\x1b[0m"
    );
}

pub fn file_error(original: &str) {
    println!("Cannot open file at {original} or {original}.bs");
}

pub fn terminated_due_to_above_errors() {
    println!("\n\x1b[37mProgram terminated due to above errors\x1b[0m");
}
