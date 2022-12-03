fn main() {
    const WINDOW_LEN: usize = 3;

    let input: Vec<u16> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    let sums: Vec<u16> = input
        .windows(WINDOW_LEN)
        .map(|x| (x.iter().sum::<u16>()))
        .collect();

    let count_increases = |count, (a, b)| if b > a { count + 1 } else { count };
    let result = sums
        .windows(2)
        .map(|win| (win[0], win[1]))
        .fold(0, count_increases);

    println!("{}", result);
}
