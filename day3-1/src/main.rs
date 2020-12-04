use std::io::{stdin, BufRead};

fn main() {
    let num_trees = stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .zip(((0 as usize)..).map(|x| x*3))
        .filter_map(|(line,x)| line
            .chars()
            .cycle()
            .nth(x)
        )
        .filter(|ch| *ch == '#')
        .count();

    println!("There are {} trees along the 1/3 slope.", num_trees);
}
