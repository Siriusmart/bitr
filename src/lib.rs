mod utils;
pub use utils::*;
mod run;
pub use run::*;
mod repl;
pub use repl::*;

pub const HELP_MSG: &str = "\x1b[32mBitr - a BitScript interpreter\x1b[0m

\x1b[37mFor more about BitScript and Bitr, visit https://siriusmart.github.io/bitscript\x1b[0m

\x1b[91mCOMMANDS:\x1b[0m
    \x1b[33mbitr\x1b[0m                 Starts the BitScript repl
    \x1b[33mbitr help\x1b[0m            Shows this message
    \x1b[33mbitr version\x1b[0m         Displays Bitr version
    \x1b[33mbitr [file name]\x1b[0m     Run from a file

\x1b[37mFile extension is not required if it is `.bs`.\x1b[0m";
