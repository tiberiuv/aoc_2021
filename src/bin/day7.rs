fn main() {
    let mut input: Vec<i32> = include_str!("../inputs/day7.txt")
        .lines()
        .next()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse().expect("valid number"))
                .collect()
        })
        .unwrap_or_default();
    input.sort_unstable();

    println!("Part A - Min gas {}", part_1(&input));
    println!("Part B - Min gas {}", part_2(&input));
}

fn part_1(numbers: &[i32]) -> i32 {
    let len = numbers.len();
    let mid = len / 2;

    numbers
        .iter()
        .fold(0, |accum, number| accum + (number - numbers[mid]).abs())
}

fn part_2(numbers: &[i32]) -> i32 {
    let mut sums: Vec<i32> = Vec::new();
    let len = numbers.len() as i32;
    for i in 0..len {
        let mut sum = 0;
        for n in numbers {
            sum += cons_sum((n - i).abs())
        }
        sums.push(sum);
    }
    *sums.iter().min().unwrap()
}

fn cons_sum(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
