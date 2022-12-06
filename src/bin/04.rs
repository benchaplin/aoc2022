fn main() {
    let input = include_str!("../../inputs/04.txt");
    pt_one(input);
    pt_two(input);
}

fn pt_one(input: &str) {
    println!("{:?}", input.split('\n')
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();

            let first_min = ranges[0].split('-').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let first_max = ranges[0].split('-').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
            let second_min = ranges[1].split('-').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let second_max = ranges[1].split('-').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

            (first_min, first_max, second_min, second_max)
        })
        .filter(|(first_min, first_max, second_min, second_max)| 
            (first_min <= second_min && first_max >= second_max) ||
            (second_min <= first_min && second_max >= first_max)
        )
        .count()
    );
}

fn pt_two(input: &str) {
    println!("{:?}", input.split('\n')
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();

            let first_min = ranges[0].split('-').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let first_max = ranges[0].split('-').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
            let second_min = ranges[1].split('-').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            let second_max = ranges[1].split('-').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

            (first_min, first_max, second_min, second_max)
        })
        .filter(|(first_min, first_max, second_min, second_max)| {
            (second_min >= first_min && second_min <= first_max) ||
            (second_max >= first_min && second_max <= first_max) ||
            (first_min <= second_min && first_max >= second_max) ||
            (second_min <= first_min && second_max >= first_max)
        })
        .count()
    );
}