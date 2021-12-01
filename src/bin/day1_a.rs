fn main() {
    let input = include_str!("../inputs/day1.txt")
        .lines()
        .map(|x| x.parse::<u16>().unwrap());

    let count_increases = |count, (a, b)| if b > a { count + 1 } else { count };
    let result = input.clone().zip(input.skip(1)).fold(0u16, count_increases);
    println!("{}", result);
}
