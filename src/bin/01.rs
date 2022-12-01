fn main() {
    let input = include_str!("../../inputs/01.txt");
    pt_one(input);
    pt_two(input);
}

fn pt_one(input: &str) {
    println!("{:?}", input.split("\n\n") 
        .map(|line| {
            line.split("\n")
                .map(|val| val.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max().unwrap()
    );
}

fn pt_two(input: &str) {
    let mut sums: Vec<i32> = input.split("\n\n") 
        .map(|line| {
            line.split("\n")
                .map(|val| val.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    sums.sort_by(|a, b| b.cmp(a));
    println!("{:?}", sums[0..3].iter().sum::<i32>());
}