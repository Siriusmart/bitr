use std::{collections::HashMap, io};

use crate::*;

pub fn input(name: &[char], status: &mut Status, radix: u32) -> Result<(), String> {
    let input = status.inputs.pop().unwrap_or_else(|| {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("cannot read line");
        input
    });

    if *name.last().unwrap_or(&' ') == ']' {
        let opening_index = match name.iter().enumerate().find(|(_index, c)| **c == '[') {
            Some((i, _c)) => i,
            None => return Err(String::from("mismatch brackets")),
        };

        input_single(
            BracketInfo {
                begin: opening_index,
                end: name.len() - 1,
                name: name[0..opening_index].iter().collect(),
                content: name[opening_index + 1..name.len() - 1].iter().collect(),
            },
            status,
            radix,
            input,
        )?;

        return Ok(());
    }
    input_multiple(
        name.iter().collect::<String>(),
        &mut status.variables,
        radix,
        input,
    )
}

fn input_single(
    info: BracketInfo,
    status: &mut Status,
    radix: u32,
    input: String,
) -> Result<(), String> {
    if let [begin, end] = info.content.split("..").collect::<Vec<_>>().as_slice() {
        let mut begin = begin.chars().collect();
        let mut end = end.chars().collect();
        eval(&mut begin, status)?;
        eval(&mut end, status)?;
        let arr = match status.variables.get_mut(&info.name) {
            Some(arr) => arr,
            None => {
                return Err(format!(
                    "cannot input to undeclared variable `{}`",
                    info.name
                ))
            }
        };

        let range = get_range_mut(
            &info.name,
            &begin.iter().collect::<String>(),
            &end.iter().collect::<String>(),
            arr,
        )?;

        assign_arr(range, input.trim(), radix)?;

        return Ok(());
    }
    let arr = match status.variables.get_mut(&info.name) {
        Some(arr) => arr,
        None => {
            return Err(format!(
                "cannot input to undeclared variable `{}`",
                info.name
            ))
        }
    };

    let index = match info.content.parse::<usize>() {
        Ok(n) => n,
        Err(_e) => return Err(format!("cannot parse `{}` into usize", info.content)),
    };

    let cell = match arr.get_mut(index) {
        Some(cell) => cell,
        None => {
            return Err(format!(
                "index out of bounds, length is {}, but index is {index}",
                arr.len()
            ))
        }
    };

    *cell = str_to_bool(input.trim())?;

    Ok(())
}

fn input_multiple(
    name: String,
    variables: &mut HashMap<String, Vec<bool>>,
    radix: u32,
    input: String,
) -> Result<(), String> {
    let arr = match variables.get_mut(&name) {
        Some(arr) => arr,
        None => return Err(format!("cannot input to undeclared variable `{name}`")),
    };

    assign_arr(arr.iter_mut().collect(), input.trim(), radix)?;

    Ok(())
}
