use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let mut sentry : HashSet<usize> = HashSet::new();
    io::stdin()
        .lock()
        .lines()
        .filter_map(|l| usize::from_str_radix(&l.ok()?, 10).ok())
        .for_each(|n| {
            let conjugate = 2020 - n;
            if sentry.contains(&conjugate) {
                println!(
                    "Found numbers: {}, {}\nProduct: {}",
                    n,
                    conjugate,
                    n * conjugate
                );
                std::process::exit(0);
            } else {
                sentry.insert(n);
            }
        });
    println!("Could not find two numbers that sum to 2020.");
}
