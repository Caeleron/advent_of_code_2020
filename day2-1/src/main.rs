use std::io::{stdin, BufRead};

#[derive(Debug)]
struct PasswordLine {
    min: usize,
    max: usize,
    policy_ch: char,
    password: String,
    ch_count: usize,
}

fn split_into_parts(s: String) -> Option<PasswordLine> {
    let mut parts = s.split_ascii_whitespace();
    let range = parts.next()?;
    let range = {
        let range_with_dash = range.split_at(range.find('-')?);
        (range_with_dash.0, range_with_dash.1.split_at(1).1)
    };
    let min = usize::from_str_radix(range.0, 10).ok()?;
    let max = usize::from_str_radix(range.1, 10).ok()?;
    let policy_ch: char = parts.next()?.chars().nth(0)?;
    let password = String::from(parts.next()?);

    if parts.next().is_none() && policy_ch.is_ascii_alphanumeric() {
        Some(PasswordLine {
            min,
            max,
            policy_ch,
            password,
            ch_count: 0,
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
        .map(|mut pl| {
            pl.ch_count = pl.password.chars().filter(|ch| *ch == pl.policy_ch).count();
            pl
        })
        .filter(|pl| pl.ch_count >= pl.min && pl.ch_count <= pl.max)
        .count();
    println!("Number of accepted password = {}", bad_line_count);
}
