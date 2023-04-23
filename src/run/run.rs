#![allow(clippy::module_inception)]

use std::io::{self, Write};

use crate::*;

pub fn run_line(line: &mut str, status: &mut Status) -> Result<Option<String>, String> {
    let code = line.split('#').next().unwrap();
    let mut code = code.trim().to_owned();

    if code.is_empty() {
        return Ok(None);
    }

    escape(&mut code);

    let mut splitted = code.split(' ').collect::<Vec<_>>();

    match splitted.as_slice() {
        ["dclr", ..] => {
            let (name, length, default) = match &splitted[1..] {
                [name, length, default] => (name, length, default),
                [name, length] => (name, length, &"0"),
                _ => return Err(String::from(
                    "incorrect usage of `dclr`, correct usage is `declr [name] [size] (default)",
                )),
            };

            let length = match length.parse::<usize>() {
                Ok(n) => n,
                Err(_e) => return Err(format!("unable to parse `{length}` as usize")),
            };

            if status
                .variables
                .insert(name.to_string(), vec![str_to_bool(default)?; length])
                .is_some()
            {
                return Err(format!("multiple declaration of variable `{name}`"));
            }
        }
        ["del", name] => {
            if status.variables.remove(*name).is_none() {
                return Err(format!("cannot delete undeclared variable `{name}`"));
            }
        }
        ["dbg"] => {
            return Err(String::from(
                "incorrect usage of `dbg`, correct usage is `dbg [expression]",
            ));
        }
        ["dbg", ..] => {
            let value_label = splitted[1..].join(" ");
            let mut value = value_label.chars().collect::<Vec<_>>();
            replace_variables(&mut value, status)?;
            eval(&mut value, status)?;
            println!("{value_label}: {}", value.into_iter().collect::<String>());
        }
        ["disp"] => {
            return Err(String::from(
                "incorrect usage of `disp`, correct usage is `disp [format string]",
            ));
        }
        ["disp", ..] => {
            let mut chars = splitted[1..].join(" ").chars().collect::<Vec<_>>();
            display_eval(&mut chars, status)?;
            print!("{}", chars.into_iter().collect::<String>());
            io::stdout().flush().expect("cannot write to stdout");
        }
        ["inpt"] => {
            return Err(String::from(
                "incorrect usage of `inpt`, correct usage is `inpt [variable] (base)`",
            ));
        }
        ["inpt", ..] => {
            let radix = if splitted.len() == 2 {
                2
            } else {
                match char_to_radix(splitted[1].chars().next().unwrap()) {
                    Ok(radix) => {
                        splitted.remove(1);
                        radix
                    }
                    Err(_) => 2,
                }
            };

            let chars = splitted[1..].join(" ").chars().collect::<Vec<_>>();
            input(&chars, status, radix)?;
        }
        ["lbl", _label] => {}
        ["goto", label] => match status.labels.get(label.to_string().as_str()) {
            Some(position) => status.line_no = *position,
            None => return Err(format!("use of undeclared label `{label}`")),
        },
        ["if", ..] => {
            let args_owner = splitted[1..].join(" ");
            let args = args_owner.splitn(2, ',').collect::<Vec<_>>();

            let mut condition = format!("AND({})", args[0]).chars().collect();
            let mut action = args[1].to_string();

            replace_variables(&mut condition, status)?;
            eval(&mut condition, status)?;

            if condition.as_slice() == ['1'] {
                return run_line(&mut action, status);
            }
        }
        ["exit", ..] => {
            let mut value = splitted[1..].join(" ").chars().collect();

            replace_variables(&mut value, status)?;
            eval(&mut value, status)?;
            return Ok(Some(value.iter().collect()));
        }
        ["reg", name, ..] => {
            if status.components.contains_key(*name) {
                return Err(format!("multiple registration of component `{name}`"));
            }

            let path = splitted[2..].join(" ");

            let (compnent_status, lines) =
                match Status::try_from_path(&path, Some(status.path.clone())) {
                    Some(status) => status,
                    None => return Err(format!("failed to load component {name}")),
                };

            status
                .components
                .insert(name.to_string(), (lines, compnent_status));
        }
        ["dereg", name] => {
            if status.components.remove(*name).is_none() {
                return Err(format!("cannot dereg unregistered component `{name}`"));
            }
        }
        _ => {
            if let Some(index) = code.find('=') {
                let (assigned, mut value) = code.split_at(index);
                value = &value[1..];

                if value.is_empty() {
                    return Err(String::from("empty right hand side value"));
                }

                let radix = match char_to_radix(value.chars().next().unwrap()) {
                    Ok(radix) => {
                        value = &value[1..];
                        radix
                    }
                    Err(_) => 2,
                };
                assign(assigned, value, status, radix)?;
                return Ok(None);
            } else {
                let mut chars = code.chars().collect();
                replace_variables(&mut chars, status)?;
                eval(&mut chars, status)?;
            }
        }
    };

    Ok(None)
}
