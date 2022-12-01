const INPUT: &str = include_str!("input.txt");

fn main() {
    let food: Vec<&str> = INPUT.split("\n\n").collect();
    let mut food: Vec<u64> = food
        .into_iter()
        .map(|s| s.lines().map(|n| n.parse::<u64>().unwrap()).sum())
        .collect();
    food.sort_by(|a, b| b.cmp(a));

    println!("1: {}", food[0]);
    println!("2: {}", food[0..=2].iter().sum::<u64>())
}
