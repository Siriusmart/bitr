use std::collections::HashMap;

use crate::*;

pub fn eval(line: &mut Vec<char>, status: &mut Status) -> Result<(), String> {
    let mut e = Ok(());

    loop {
        let bracket_info = match find_brackets('(', ')', line) {
            Ok(Some(info)) => info,
            Ok(None) => break,
            Err(e) => return Err(e),
        };

        e.as_ref()?;

        let replace = if let Some(lines) = status.components.get(&bracket_info.name) {
            let (lines, mut component_status) = lines.clone();
            component_status.inputs = bracket_info
                .content
                .split(',')
                .map(str::to_string)
                .collect();

            let mut value = String::new();
            while component_status.line_no < lines.len() {
                match run_line(
                    &mut lines[component_status.line_no].clone(),
                    &mut component_status,
                ) {
                    Ok(None) => {}
                    Ok(Some(val)) => {
                        value = val;
                        break;
                    }
                    Err(e) => {
                        file_terminated(
                            &e,
                            component_status.line_no + 1,
                            &lines[component_status.line_no],
                        );
                        return Err(format!(
                            "error when executing component `{}`",
                            bracket_info.name
                        ));
                    }
                }

                component_status.line_no += 1;
            }

            value
        } else {
            match bracket_info.name.as_str() {
                "HEX" => format!("{:X}", to_denary(&str_to_bools(&bracket_info.content)?)),
                "DEN" => to_denary(&str_to_bools(&bracket_info.content)?).to_string(),
                "OCT" => format!("{:o}", to_denary(&str_to_bools(&bracket_info.content)?)),
                _ => {
                    let params = bracket_info
                        .content
                        .chars()
                        .filter(|&c| c == '0' || c == '1')
                        .map(|cell| match char_to_bool(cell) {
                            Ok(cell) => cell,
                            Err(err) => {
                                e = Err(err);
                                false
                            }
                        })
                        .collect::<Vec<_>>();
                    run_gate(&bracket_info.name, &params)?
                }
            }
        };

        line.splice(bracket_info.begin..bracket_info.end + 1, replace.chars());
    }

    Ok(())
}

pub fn display_eval(line: &mut Vec<char>, status: &mut Status) -> Result<(), String> {
    loop {
        let bracket_info = match find_brackets('{', '}', line) {
            Ok(Some(info)) => info,
            Ok(None) => break,
            Err(e) => return Err(e),
        };

        let mut content = bracket_info.content.chars().collect::<Vec<_>>();
        replace_variables(&mut content, status)?;
        eval(&mut content, status)?;

        line.splice(bracket_info.begin..bracket_info.end + 1, content);
    }

    if line.first() == Some(&'\'') && line.last() == Some(&'\'') && line.len() > 1 {
        line.pop();
        line.remove(0);
    }

    Ok(())
}

pub fn replace_variables(chars: &mut Vec<char>, status: &mut Status) -> Result<(), String> {
    while let Some(bracket_info) = find_brackets('[', ']', chars)? {
        let value = if let [begin, end] = bracket_info
            .content
            .split("..")
            .collect::<Vec<_>>()
            .as_slice()
        {
            let mut begin = begin.chars().collect();
            let mut end = end.chars().collect();
            eval(&mut begin, status)?;
            eval(&mut end, status)?;
            let arr = match status.variables.get(&bracket_info.name) {
                Some(arr) => arr,
                None => {
                    return Err(format!(
                        "use of undeclared variable `{}`",
                        bracket_info.name
                    ))
                }
            };

            bools_to_string(
                &get_range(
                    &bracket_info.name,
                    &begin.iter().collect::<String>(),
                    &end.iter().collect::<String>(),
                    arr,
                )?
                .into_iter()
                .rev()
                .copied()
                .collect::<Vec<_>>(),
            )
        } else {
            let index: usize = parse_n(&bracket_info.content)?;
            let arr = match status.variables.get(&bracket_info.name) {
                Some(arr) => arr,
                None => {
                    return Err(format!(
                        "use of undeclared variable `{}`",
                        bracket_info.name
                    ))
                }
            };
            match arr.get(index) {
                Some(&value) => (value as u8).to_string(),
                None => {
                    return Err(format!(
                        "index out of bounds, `{}` is only `{}` long, but the index is {index}",
                        bracket_info.name,
                        arr.len()
                    ))
                }
            }
        };

        chars.splice(bracket_info.begin..bracket_info.end + 1, value.chars());
    }

    let mut s = chars.iter().collect::<String>();
    replace_more_variables(&mut s, &status.variables)?;
    *chars = s.chars().collect::<Vec<_>>();

    Ok(())
}

fn replace_more_variables(
    s: &mut String,
    variables: &HashMap<String, Vec<bool>>,
) -> Result<(), String> {
    variables.iter().for_each(|(name, value)| {
        let mut base_index = 0;

        while let Some(mut index) = s[base_index..].find(name) {
            index += base_index;

            let end_index = s
                .chars()
                .enumerate()
                .skip(index)
                .find(|(_index, c)| !(c.is_ascii_alphabetic() || *c == '_'))
                .unwrap_or((s.len(), ' '))
                .0;

            if name == &s[index..end_index] {
                s.replace_range(
                    index..end_index,
                    value
                        .iter()
                        .map(|&b| (b as u8).to_string())
                        .rev()
                        .collect::<String>()
                        .as_str(),
                );
                base_index = end_index + value.len() - name.len();
            } else {
                base_index = end_index;
            }
        }
    });

    Ok(())
}
