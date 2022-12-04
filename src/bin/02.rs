use core::panic;

fn main() {
    let input = include_str!("../../inputs/02.txt");
    pt_one(input);
    pt_two(input);
}

fn pt_one(input: &str) {
    println!("{:?}", input.split('\n')
        .map(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            let elf_val = match moves[0] {
                "A" => { 1 } // rock
                "B" => { 2 } // paper
                "C" => { 3 } // scissors
                &_ => { panic!("bad val") }
            };
            let my_val = match moves[1] {
                "X" => { 1 } // rock
                "Y" => { 2 } // paper
                "Z" => { 3 } // scissors
                &_ => { panic!("bad val") }
            };

            let mut score = my_val;
            if (my_val - elf_val + 3) % 3 == 1 {
                score += 6;
            } else if my_val == elf_val {
                score += 3;
            }
            
            score
        })
        .sum::<i32>());
}

fn pt_two(input: &str) {
    println!("{:?}", input.split('\n')
        .map(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            let elf_val = match moves[0] {
                "A" => { 1 } // rock
                "B" => { 2 } // paper
                "C" => { 3 } // scissors
                &_ => { panic!("bad val") }
            };
            let my_res = match moves[1] {
                "X" => { 0 } // lose
                "Y" => { 3 } // draw 
                "Z" => { 6 } // win 
                &_ => { panic!("bad val") }
            };

            let mut score = my_res;
            if my_res == 0 {
                score += ((elf_val - 1) + 2) % 3 + 1;
            } else if my_res == 3 {
                score += elf_val;
            } else {
                score += ((elf_val - 1) + 1) % 3 + 1;
            }
            
            score
        })
        .sum::<i32>());
}