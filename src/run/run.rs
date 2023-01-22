#![allow(clippy::module_inception)]

use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{
    assign, char_to_radix, display_eval, escape, eval, input, replace_variables, str_to_bool,
};

pub fn run_line(
    line: &mut String,
    variables: &mut HashMap<String, Vec<bool>>,
) -> Result<(), String> {
    let (code, _comment) = line.split_at(line.find('#').unwrap_or(line.len()));
    let mut code = code.trim().to_owned();

    if code.is_empty() {
        return Ok(());
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

            if variables
                .insert(name.to_string(), vec![str_to_bool(default)?; length])
                .is_some()
            {
                return Err(format!("multiple declaration of variable `{name}`"));
            }

            return Ok(());
        }
        ["dbg"] => {
            return Err(String::from(
                "incorrect usage of `dbg`, correct usage is `dbg [expression]",
            ));
        }
        ["dbg", ..] => {
            let value_label = splitted[1..].join(" ");
            let mut value = value_label.chars().collect::<Vec<_>>();
            replace_variables(&mut value, variables)?;
            eval(&mut value)?;
            println!("{value_label}: {}", value.into_iter().collect::<String>());
        }
        ["disp"] => {
            return Err(String::from(
                "incorrect usage of `disp`, correct usage is `disp [format string]",
            ));
        }
        ["disp", ..] => {
            let mut chars = splitted[1..].join(" ").chars().collect::<Vec<_>>();
            display_eval(&mut chars, variables)?;
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
            input(&chars, variables, radix)?;
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
                assign(assigned, value, variables, radix)?;
                return Ok(());
            }
            return Err(format!("unrecognised pattern `{code}`"));
        }
    };

    Ok(())
}
