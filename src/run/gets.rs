use crate::parse_n;

pub fn get_range_mut<'a>(
    name: &str,
    begin: &str,
    end: &str,
    arr: &'a mut Vec<bool>,
) -> Result<Vec<&'a mut bool>, String> {
    let begin: usize = if begin.is_empty() { 0 } else { parse_n(begin)? };
    let end: usize = if end.is_empty() {
        arr.len()
    } else {
        parse_n(end)?
    };

    if begin > end {
        return Err(format!(
            "beginning index `{begin}` cannot be larger than ending index `{end}`"
        ));
    }

    if end > arr.len() {
        return Err(format!(
            "index out of bounds, `{}` is only `{}` long, but the ending index is {end}",
            name,
            arr.len()
        ));
    };

    Ok(arr
        .iter_mut()
        .skip(begin)
        .take(end - begin)
        .collect::<Vec<_>>())
}

pub fn get_range<'a>(
    name: &str,
    begin: &str,
    end: &str,
    arr: &'a Vec<bool>,
) -> Result<Vec<&'a bool>, String> {
    let begin: usize = if begin.is_empty() { 0 } else { parse_n(begin)? };
    let end: usize = if end.is_empty() {
        arr.len()
    } else {
        parse_n(end)?
    };

    if begin > end {
        return Err(format!(
            "beginning index `{begin}` cannot be larger than ending index `{end}`"
        ));
    }

    if end > arr.len() {
        return Err(format!(
            "index out of bounds, `{}` is only `{}` long, but the ending index is {end}",
            name,
            arr.len()
        ));
    };

    Ok(arr.iter().skip(begin).take(end - begin).collect::<Vec<_>>())
}
