use crate::bools_to_string;

pub fn run_gate(name: &str, args: &[bool]) -> Result<String, String> {
    let res = match name {
        "NOT" => not(args),
        "AND" => and(args),
        "OR" => or(args),
        "NOR" => nor(args),
        "NAND" => nand(args),
        "XOR" => xor(args),
        "XNOR" => xnor(args),
        _ => return Err(format!("unknown gate `{name}`")),
    };

    Ok(bools_to_string(&res))
}

fn not(args: &[bool]) -> Vec<bool> {
    args.iter().map(|&b| !b).collect()
}

fn and(args: &[bool]) -> Vec<bool> {
    vec![args.iter().all(|&b| b)]
}

fn or(args: &[bool]) -> Vec<bool> {
    vec![args.iter().any(|&b| b)]
}

fn nor(args: &[bool]) -> Vec<bool> {
    not(&or(args))
}

fn nand(args: &[bool]) -> Vec<bool> {
    not(&and(args))
}

fn xor(args: &[bool]) -> Vec<bool> {
    vec![!xnor(args)[0]]
}

fn xnor(args: &[bool]) -> Vec<bool> {
    vec![args.iter().filter(|&&b| b).count() % 2 == 0]
}
