fn get_op(line: &str) -> (&str, i64) {
    let mut split = line.split(' ');
    let direction = split.next().unwrap_or_default();

    let number = split
        .next()
        .map(str::parse::<i64>)
        .and_then(Result::ok)
        .unwrap_or(0);
    (direction, number)
}

fn main() {
    let fold_line = |(aim, depth, position), line: &str| {
        let (direction, number) = get_op(line);

        let (p, a) = match direction {
            "forward" => (1, 0),
            "up" => (0, -1),
            "down" => (0, 1),
            _ => (0, 0),
        };

        (
            aim + (number * a),
            depth + (number * p * aim),
            position + (number * p),
        )
    };

    let (_, depth, position) = include_str!("../inputs/day2.txt")
        .lines()
        .fold((0, 0, 0), fold_line);

    println!("{}", depth * position);
}
