use std::{collections::HashMap, io};

use crate::{assign_arr, eval, get_range_mut, str_to_bool, BracketInfo};

pub fn input(
    name: &[char],
    variables: &mut HashMap<String, Vec<bool>>,
    radix: u32,
) -> Result<(), String> {
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
            variables,
            radix,
        )?;

        return Ok(());
    }
    input_multiple(name.iter().collect::<String>(), variables, radix)
}

fn input_single(
    info: BracketInfo,
    variables: &mut HashMap<String, Vec<bool>>,
    radix: u32,
) -> Result<(), String> {
    let arr = match variables.get_mut(&info.name) {
        Some(arr) => arr,
        None => {
            return Err(format!(
                "cannot input to undeclared variable `{}`",
                info.name
            ))
        }
    };

    if let [begin, end] = info.content.split("..").collect::<Vec<_>>().as_slice() {
        let mut begin = begin.chars().collect();
        let mut end = end.chars().collect();
        eval(&mut begin)?;
        eval(&mut end)?;

        let range = get_range_mut(
            &info.name,
            &begin.iter().collect::<String>(),
            &end.iter().collect::<String>(),
            arr,
        )?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("cannot read line");

        assign_arr(range, input.trim(), radix)?;

        return Ok(());
    }

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

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");

    *cell = str_to_bool(input.trim())?;

    Ok(())
}

fn input_multiple(
    name: String,
    variables: &mut HashMap<String, Vec<bool>>,
    radix: u32,
) -> Result<(), String> {
    let arr = match variables.get_mut(&name) {
        Some(arr) => arr,
        None => return Err(format!("cannot input to undeclared variable `{}`", name)),
    };

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");

    assign_arr(arr.iter_mut().collect(), input.trim(), radix)?;

    Ok(())
}
