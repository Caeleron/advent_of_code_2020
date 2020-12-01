use std::io::{self, BufRead};
use std::collections::HashSet;

fn find_conjugate_pair(input: &Vec<usize>, target: usize) -> Option<(usize, usize)> {
    let sum = 2020 - target;
    let mut sentry : HashSet<usize> = HashSet::new();
    let mut pairs = input
        .iter()
        .copied()
        .filter(|n| *n < sum)
        .filter_map(|n| {
            let conjugate = sum - n;
            if sentry.contains(&conjugate) {
                Some((n, conjugate))
            } else {
                sentry.insert(n);
                None
            }
        });
    pairs.next()
}

fn main() {
    let input: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| usize::from_str_radix(&l.ok()?, 10).ok())
        .collect();

    input.iter().for_each(|n| {
        if let Some((a, b)) = find_conjugate_pair(&input, *n) {
            println!("Found numbers: {}, {}, {}\nProduct: {}", a, b, n, a * b * n);
            std::process::exit(0);
        }
    });

    println!("Could not find three numbers that sum to 2020.");
}
