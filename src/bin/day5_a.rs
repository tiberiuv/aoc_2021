fn parse_opt_num(item: Option<&str>) -> i32 {
    item.and_then(|x| x.parse().ok()).unwrap_or_default()
}

const GRID_SIZE: usize = 10;

fn main() {
    let mut grid = [0_i32; GRID_SIZE * GRID_SIZE];

    include_str!("../inputs/day5.txt")
        .lines()
        .map(|l| {
            let pairs: Vec<(i32, i32)> = l
                .split(" -> ")
                .map(|l| {
                    let mut numbers = l.split(',');
                    let x: i32 = parse_opt_num(numbers.next());
                    let y: i32 = parse_opt_num(numbers.next());
                    (x, y)
                })
                .collect();
            (pairs[0], pairs[1])
        })
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|(a, b)| {
            let points = get_line_points(a, b);
            for (x, y) in points {
                let idx = (x + (GRID_SIZE as i32 * y)) as usize;
                grid[idx] += 1;
            }
        });
    let count = grid.iter().filter(|x| **x > 1).count();

    for (idx, x) in grid.iter().enumerate() {
        print!("{} |", x);
        if (idx + 1) % GRID_SIZE == 0 {}
    }

    println!("\n Final count {}", count);
}

fn get_line_points(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let mut x = x1;
    let mut y = y1;

    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    let mut points = Vec::from([(x, y)]);
    while !(x == x2 && y == y2) {
        x += dx;
        y += dy;
        points.push((x, y));
    }

    points
}
