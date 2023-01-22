use std::str::FromStr;

pub fn str_to_bool(value: &str) -> Result<bool, String> {
    match value.trim() {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(format!("unable to convert `{value}` into a binary value")),
    }
}

pub fn char_to_bool(value: char) -> Result<bool, String> {
    match value {
        '0' => Ok(false),
        '1' => Ok(true),
        _ => Err(format!("unable to convert `{value}` into a binary value")),
    }
}

pub fn str_to_bools(value: &str) -> Result<Vec<bool>, String> {
    let mut bools = Vec::with_capacity(value.len());

    for c in value.chars() {
        bools.push(char_to_bool(c)?);
    }

    Ok(bools)
}

pub fn bools_to_string(values: &[bool]) -> String {
    values
        .iter()
        .map(|&b| (b as u8).to_string())
        .collect::<String>()
}

pub fn char_to_radix(c: char) -> Result<u32, String> {
    match c.to_ascii_lowercase() {
        'o' => Ok(8),
        'h' => Ok(16),
        'd' => Ok(10),
        'b' => Ok(2),
        _ => Err(format!("invalid radix `{c}`")),
    }
}

pub fn parse_n<T>(s: &str) -> Result<T, String>
where
    T: FromStr,
{
    match s.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_e) => Err(format!("cannot parse `{} as usize`", s)),
    }
}

pub fn find_brackets(
    opening: char,
    closing: char,
    chars: &[char],
) -> Result<Option<BracketInfo>, String> {
    if chars.iter().filter(|c| **c == opening).count()
        != chars.iter().filter(|c| **c == closing).count()
    {
        return Err(format!("mismatch amound of `{opening}` and `{closing}`"));
    }

    let bracket_end = match chars.iter().enumerate().find(|(_index, c)| **c == closing) {
        Some((index, _c)) => index,
        None => return Ok(None),
    };

    let bracket_begin = match chars
        .iter()
        .enumerate()
        .rev()
        .skip(chars.len() - bracket_end)
        .find(|(_index, c)| **c == opening)
    {
        Some((index, _c)) => index,
        None => return Err(String::from("mismatch brackets")),
    };

    let operator_begin = match chars
        .iter()
        .enumerate()
        .take(bracket_begin)
        .rev()
        .find(|(_index, c)| !(c.is_ascii_alphabetic() || **c == '_'))
    {
        Some((index, _c)) => index + 1,
        None => 0,
    };

    let operator = chars[operator_begin..bracket_begin]
        .iter()
        .collect::<String>();

    let operands = chars[bracket_begin + 1..bracket_end]
        .iter()
        .collect::<String>();

    Ok(Some(BracketInfo {
        begin: operator_begin,
        end: bracket_end,
        name: operator,
        content: operands,
    }))
}

#[derive(Debug)]
pub struct BracketInfo {
    pub begin: usize,
    pub end: usize,
    pub name: String,
    pub content: String,
}

pub fn escape(s: &mut String) {
    *s = s.replace("\\n", "\n");
}

pub fn to_denary(bools: &[bool]) -> u64 {
    bools
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &b)| if b { 2_u64.pow(index as u32) } else { 0 })
        .sum()
}
