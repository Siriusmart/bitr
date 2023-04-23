use crate::*;

// None => error
// Some(None) => no return value
// Some(T) => return value
pub fn run_script(mut status: Status, lines: Vec<String>) -> Option<Option<String>> {
    while status.line_no < lines.len() {
        match run_line(&mut lines[status.line_no].clone(), &mut status) {
            Ok(None) => {}
            Ok(Some(value)) => {
                return Some(Some(value));
            }
            Err(e) => {
                file_terminated(&e, status.line_no + 1, &lines[status.line_no]);
                return None;
            }
        }

        status.line_no += 1;
    }

    Some(None)
}
