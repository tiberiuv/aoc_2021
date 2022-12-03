fn main() {
    let input: Vec<u16> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    let count_increases = |count, (a, b)| if b > a { count + 1 } else { count };
    let result = input
        .windows(2)
        .map(|nums| (nums[0], nums[1]))
        .fold(0u16, count_increases);

    println!("{}", result);
}
