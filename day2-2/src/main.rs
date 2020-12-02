use std::io::{stdin, BufRead};

#[derive(Debug)]
struct PasswordLine {
    pos1: usize,
    pos2: usize,
    policy_ch: char,
    password: String,
}

fn split_into_parts(s: String) -> Option<PasswordLine> {
    let mut parts = s.split_ascii_whitespace();
    let range = parts.next()?;
    let range = {
        let range_with_dash = range.split_at(range.find('-')?);
        (range_with_dash.0, range_with_dash.1.split_at(1).1)
    };
    let pos1 = usize::from_str_radix(range.0, 10).ok()?;
    let pos2 = usize::from_str_radix(range.1, 10).ok()?;
    let policy_ch: char = parts.next()?.chars().nth(0)?;
    let password = String::from(parts.next()?);

    if parts.next().is_none() && policy_ch.is_ascii_alphanumeric() {
        Some(PasswordLine {
            pos1,
            pos2,
            policy_ch,
            password,
        })
    } else {
        None
    }
}

fn main() {
    let bad_line_count = stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(split_into_parts)
        .filter(|pl| {
            let char1 = pl.password.chars().nth(pl.pos1 - 1);
            let char2 = pl.password.chars().nth(pl.pos2 - 1);
            println!(
                "({},{}) in {} = ({},{})",
                pl.pos1,
                pl.pos2,
                pl.password,
                char1.unwrap_or('∅'),
                char2.unwrap_or('∅')
            );
            (char1 == Some(pl.policy_ch)) ^ (char2 == Some(pl.policy_ch))
        })
        .count();
    println!("Number of accepted password = {}", bad_line_count);
}
