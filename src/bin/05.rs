fn main() {
    let input = include_str!("../../inputs/05.txt");
    pt_one(input);
    println!("");
    pt_two(input);
}

fn pt_one(input: &str) {
    let s1 = vec!['C', 'Z', 'N', 'B', 'M', 'W', 'Q', 'V'];
    let s2 = vec!['H', 'Z', 'R', 'W', 'C', 'B'];
    let s3 = vec!['F', 'Q', 'R', 'J'];
    let s4 = vec!['Z', 'S', 'W', 'H', 'F', 'N', 'M', 'T'];
    let s5 = vec!['G', 'F', 'W', 'L', 'N', 'Q', 'P'];
    let s6 = vec!['L', 'P', 'W'];
    let s7 = vec!['V', 'B', 'D', 'R', 'G', 'C', 'Q', 'J'];
    let s8 = vec!['Z', 'Q', 'N', 'B', 'W'];
    let s9 = vec!['H', 'L', 'F', 'C', 'G', 'T', 'J'];
    let mut stacks: Vec<Vec<char>> = vec![s1, s2, s3, s4, s5, s6, s7, s8, s9];

    let instructions = input.split('\n')
        .skip(10)
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            let move_num = split[1].parse::<usize>().unwrap();
            let from = split[3].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();

            (move_num, from, to)
        });

    for (move_num, from, to) in instructions {
        for _ in 0..move_num {
            let creight = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(creight);
        }
    }

    for stack in stacks {
        print!("{}", stack.get(stack.len() - 1).unwrap());
    }
}

fn pt_two(input: &str) {
    let s1 = vec!['C', 'Z', 'N', 'B', 'M', 'W', 'Q', 'V'];
    let s2 = vec!['H', 'Z', 'R', 'W', 'C', 'B'];
    let s3 = vec!['F', 'Q', 'R', 'J'];
    let s4 = vec!['Z', 'S', 'W', 'H', 'F', 'N', 'M', 'T'];
    let s5 = vec!['G', 'F', 'W', 'L', 'N', 'Q', 'P'];
    let s6 = vec!['L', 'P', 'W'];
    let s7 = vec!['V', 'B', 'D', 'R', 'G', 'C', 'Q', 'J'];
    let s8 = vec!['Z', 'Q', 'N', 'B', 'W'];
    let s9 = vec!['H', 'L', 'F', 'C', 'G', 'T', 'J'];
    let mut stacks: Vec<Vec<char>> = vec![s1, s2, s3, s4, s5, s6, s7, s8, s9];

    let instructions = input.split('\n')
        .skip(10)
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            let move_num = split[1].parse::<usize>().unwrap();
            let from = split[3].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();

            (move_num, from, to)
        });

    for (move_num, from, to) in instructions {
        let from_stack = stacks[from - 1].clone();
        let creights = &from_stack[(from_stack.len() - move_num)..from_stack.len()];
        for _ in 0..move_num {
            stacks[from - 1].pop();
        }
        for creight in creights {
            stacks[to - 1].push(creight.clone());
        }
    }

    for stack in stacks {
        print!("{}", stack.get(stack.len() - 1).unwrap());
    }
}