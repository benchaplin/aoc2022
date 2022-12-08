use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/06.txt");
    pt_one(input);
    pt_two(input);
}

fn pt_one(input: &str) {
    let chars: Vec<char> = input.chars().collect();

    for i in 3..chars.len() {
        let last_four: HashSet<&char> = HashSet::from_iter(
            &chars[(i - 3)..(i + 1)]
        );
        if last_four.len() == 4 {
            println!("{}", i + 1);
            return;
        }
    }
}

fn pt_two(input: &str) {
    let chars: Vec<char> = input.chars().collect();

    for i in 13..chars.len() {
        let last_fourteen: HashSet<&char> = HashSet::from_iter(
            &chars[(i - 13)..(i + 1)]
        );
        if last_fourteen.len() == 14 {
            println!("{}", i + 1);
            return;
        }
    }
}