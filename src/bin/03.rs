use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/03.txt");
    pt_one(input);
    pt_two(input);
}

fn get_char_val(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 64 + 26;
    }
    c as u32 - 96
}

fn pt_one(input: &str) {
    println!("{:?}", input.split('\n')
        .map(|line| {
            let first_half: HashSet<char> = line.chars()
                .enumerate()
                .filter(|&(i, _)| i < line.len() / 2)
                .map(|(_, e)| e)
                .collect();
            let second_half: HashSet<char> = line.chars()
                .enumerate()
                .filter(|&(i, _)| i >= line.len() / 2)
                .map(|(_, e)| e)
                .collect();
            
            let common = &first_half & &second_half;

            get_char_val(common.iter().next().unwrap().clone())
        })
        .sum::<u32>()
    );
}

fn pt_two(input: &str) {
    println!("{:?}", input.split('\n')
        .tuples()
        .map(|(line1, line2, line3)| {
            let line1: HashSet<char> = line1.chars().collect();
            let line2: HashSet<char> = line2.chars().collect();
            let line3: HashSet<char> = line3.chars().collect();

            let common = &(&line1 & &line2) & &line3; 

            get_char_val(common.iter().next().unwrap().clone())
        })
        .sum::<u32>()
    );
}
