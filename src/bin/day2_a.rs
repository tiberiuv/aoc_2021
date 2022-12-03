fn get_op(line: &str) -> (&str, i64) {
    let mut split = line.split(' ');
    let direction = split.next().unwrap_or_default();

    let number = split
        .next()
        .map(str::parse::<i64>)
        .and_then(Result::ok)
        .expect("invalid input");

    (direction, number)
}

fn main() {
    let fold_line = |(position, depth), line: &str| {
        let (direction, number) = get_op(line);
        let (x1, y1) = match direction {
            "forward" => (1, 0),
            "up" => (0, -1),
            "down" => (0, 1),
            _ => panic!("unexpected input"),
        };

        (position + x1 * number, depth + y1 * number)
    };

    let (position, depth) = include_str!("../inputs/day2.txt")
        .lines()
        .map(str::trim)
        .fold((0, 0), fold_line);

    println!("{} | {}", position, depth);
    println!("{}", position * depth);
}
