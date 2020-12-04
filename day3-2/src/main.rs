use std::io::{stdin, Read};

fn count_trees(input: &String, rise: usize, run: usize) -> usize {
    let num_trees = input
        .lines()
        .step_by(rise)
        .zip(((0 as usize)..).map(|x| x*run))
        .filter_map(|(str,x)| str.chars().cycle().nth(x))
        .filter(|ch| *ch == '#')
        .count();

    println!("There are {} trees along the {}/{} slope.", num_trees, rise, run);
    num_trees
}

fn main() {
    let mut input_str = String::new();
    stdin().lock().read_to_string(&mut input_str).expect("Error reading from stdin.\n");

    let slopes : Vec<(usize,usize)> = vec![(1,1),(1,3),(1,5),(1,7),(2,1)];
    let product : usize = slopes
        .iter()
        .map(|(rise,run)|count_trees(&input_str, *rise, *run))
        .product();

    println!("Product of all slopes: {}", product);
}
