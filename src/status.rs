use std::{collections::HashMap, fs, path::PathBuf};

use crate::*;

#[derive(Default, Clone)]
pub struct Status {
    pub variables: HashMap<String, Vec<bool>>,
    pub labels: HashMap<String, usize>,
    pub components: HashMap<String, (Vec<String>, Status)>,
    pub line_no: usize,
    pub inputs: Vec<String>,
    pub path: PathBuf,
}

impl Status {
    // Some(T) => Ok
    // None => Err
    pub fn try_from_path(path: &str, base: Option<PathBuf>) -> Option<(Self, Vec<String>)> {
        let base = match base {
            Some(mut path) => {
                path.pop();
                path
            }
            None => PathBuf::new(),
        };

        let path = PathBuf::from(path);
        if path.starts_with("std") {}

        let path = base.join(path);

        let script = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => {
                let mut new_path = path.clone();
                new_path.set_extension("bs");
                match fs::read_to_string(&new_path) {
                    Ok(s) => s,
                    Err(_) => {
                        file_error(path.to_str().unwrap());
                        return None;
                    }
                }
            }
        };

        let mut out = Self {
            path,
            ..Self::default()
        };

        let mut multiple_definitions_terminate = false;
        let lines = script
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if let ["lbl", label] = line.split(' ').collect::<Vec<_>>().as_slice() {
                    let label_string = label.to_string();
                    match out.labels.get(&label_string) {
                        Some(occupied) => {
                            file_terminated(
                                &multiple_definitions_lbl(label, *occupied + 1),
                                index + 1,
                                line,
                            );
                            multiple_definitions_terminate = true;
                        }
                        None => {
                            out.labels.insert(label_string, index);
                        }
                    }
                }
                line.trim().to_string()
            })
            .collect::<Vec<_>>();

        if multiple_definitions_terminate {
            return None;
        }

        Some((out, lines))
    }
}
