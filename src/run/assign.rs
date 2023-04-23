use std::collections::HashMap;

use crate::*;

pub fn assign(assigned: &str, value: &str, status: &mut Status, radix: u32) -> Result<(), String> {
    let assigned = assigned
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>();
    let mut value = value
        .chars()
        .skip(1)
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>();

    replace_variables(&mut value, status)?;
    eval(&mut value, status)?;
    let value = value.iter().collect::<String>();

    if *assigned.last().unwrap_or(&' ') == ']' {
        let opening_index = match assigned.iter().enumerate().find(|(_index, c)| **c == '[') {
            Some((i, _c)) => i,
            None => return Err(String::from("mismatch brackets")),
        };

        assign_single(
            BracketInfo {
                begin: opening_index,
                end: assigned.len() - 1,
                name: assigned[0..opening_index].iter().collect(),
                content: assigned[opening_index + 1..assigned.len() - 1]
                    .iter()
                    .collect(),
            },
            &value,
            status,
            radix,
        )?;
        return Ok(());
    }

    assign_multiple(
        assigned.iter().collect::<String>().as_str(),
        &value,
        &mut status.variables,
        radix,
    )
}

pub fn assign_arr(mut assign_to: Vec<&mut bool>, value: &str, radix: u32) -> Result<(), String> {
    let mut e = Ok(());

    if radix == 2 {
        for (cell, value) in assign_to.iter_mut().zip(value.chars().rev()) {
            **cell = char_to_bool(value)?;
        }

        assign_to
            .iter_mut()
            .skip(value.len())
            .for_each(|cell| **cell = false);
        return Ok(());
    }

    let value = value
        .chars()
        .rev()
        .enumerate()
        .map(|(index, c)| {
            (radix as u64).pow(index as u32)
                * c.to_digit(radix).unwrap_or_else(|| {
                    e = Err(format!("invalid character `{c}` in based {radix} number"));
                    0
                }) as u64
        })
        .sum::<u64>();

    e?;

    let mut binary_value = format!("{value:b}");
    binary_value = format!(
        "{}{binary_value}",
        "0".repeat(if assign_to.len() > binary_value.len() {
            assign_to.len() - binary_value.len()
        } else {
            0
        })
    );

    for (cell, value) in assign_to.iter_mut().zip(binary_value.chars().rev()) {
        **cell = char_to_bool(value)?;
    }

    Ok(())
}

fn assign_multiple(
    assigned: &str,
    value: &str,
    variables: &mut HashMap<String, Vec<bool>>,
    radix: u32,
) -> Result<(), String> {
    let arr = match variables.get_mut(assigned) {
        Some(arr) => arr,
        None => return Err(format!("cannot assign to undeclared variable `{assigned}`")),
    };

    assign_arr(arr.iter_mut().collect(), value, radix)?;

    Ok(())
}

fn assign_single(
    mut bracket_info: BracketInfo,
    value: &str,
    status: &mut Status,
    radix: u32,
) -> Result<(), String> {
    let mut chars = bracket_info.content.chars().collect::<Vec<_>>();
    replace_variables(&mut chars, status)?;
    eval(&mut chars, status)?;
    bracket_info.content = chars.iter().collect();

    if let [begin, end] = bracket_info
        .content
        .split("..")
        .collect::<Vec<_>>()
        .as_slice()
    {
        let mut begin = begin.chars().collect();
        let mut end = end.chars().collect();
        eval(&mut begin, status)?;
        eval(&mut end, status)?;
        let arr = match status.variables.get_mut(&bracket_info.name) {
            Some(arr) => arr,
            None => {
                return Err(format!(
                    "cannot use undeclared variable `{}`",
                    bracket_info.name
                ))
            }
        };

        let range = get_range_mut(
            &bracket_info.name,
            &begin.iter().collect::<String>(),
            &end.iter().collect::<String>(),
            arr,
        )?;
        assign_arr(range, value, radix)?;
        return Ok(());
    }
    let mut value = value.chars().collect::<Vec<_>>();
    eval(&mut value, status)?;
    let arr = match status.variables.get_mut(&bracket_info.name) {
        Some(arr) => arr,
        None => {
            return Err(format!(
                "cannot use undeclared variable `{}`",
                bracket_info.name
            ))
        }
    };

    let index = match bracket_info.content.parse::<usize>() {
        Ok(n) => n,
        Err(_e) => {
            return Err(format!(
                "cannot parse `{}` into usize",
                bracket_info.content
            ))
        }
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

    *cell = str_to_bool(&value.iter().collect::<String>())?;

    Ok(())
}

// fn assign_range()
