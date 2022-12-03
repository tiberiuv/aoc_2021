fn parse_opt_num(item: Option<&str>) -> usize {
    item.and_then(|x| x.parse().ok()).unwrap_or_default()
}

const GRID_SIZE: usize = 1000;

fn main() {
    let mut grid = [0_u8; GRID_SIZE * GRID_SIZE];

    include_str!("../inputs/day5.txt")
        .lines()
        .map(|l| {
            let pairs: Vec<(usize, usize)> = l
                .split(" -> ")
                .map(|x| {
                    let mut numbers = x.split(',');
                    let a1: usize = parse_opt_num(numbers.next());
                    let a2: usize = parse_opt_num(numbers.next());
                    (a1, a2)
                })
                .collect();
            (pairs[0], pairs[1])
        })
        .for_each(|(a, b)| {
            let points = get_line_indexes(a, b);
            for (x, y) in points {
                let idx = x + (GRID_SIZE * y);
                grid[idx] += 1;
            }
        });
    let count = grid.iter().filter(|x| **x > 1).count();

    for (idx, x) in grid.iter().enumerate() {
        print!("{} |", x);
        if (idx + 1) % GRID_SIZE == 0 {
            println!();
        }
    }

    println!("Count {}", count);
}

fn get_line_indexes(a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
    let (x1, y1) = a;
    let (x2, y2) = b;
    if x1 == x2 {
        if y1 > y2 { y2..=y1 } else { y1..=y2 }
            .map(|y| (x1, y))
            .collect()
    } else if y1 == y2 {
        if x1 > x2 { x2..=x1 } else { x1..=x2 }
            .map(|x| (x, y1))
            .collect()
    } else {
        let xs = (x2..=x1).chain((x1..=x2).rev());
        let ys = (y2..=y1).chain((y1..=y2).rev());
        xs.zip(ys).collect()
    }
}
