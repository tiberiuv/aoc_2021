use std::collections::HashSet;
use std::str;

const ROW_LENGTH: i64 = 10;

#[derive(Debug)]
pub struct Grid {
    data: Vec<u8>,
    flashes: usize,
}

impl Grid {
    fn inc_all(&mut self) {
        for x in &mut self.data {
            *x += 1;
        }
    }
    fn inc_indexes(&mut self, indexes: &[usize]) {
        for idx in indexes {
            *self.data.get_mut(*idx).unwrap() += 1;
        }
    }
    fn reset_indexes(&mut self, indexes: impl IntoIterator<Item = usize>) {
        for idx in indexes {
            *self.data.get_mut(idx).unwrap() = 0;
        }
    }
    fn should_flash(&self, flashed: &HashSet<usize>) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| {
                if *x > 9 && !flashed.contains(&idx) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    fn flash_all(&mut self) {
        let mut flashed = HashSet::<usize>::new();

        loop {
            let idx_should_flash = self.should_flash(&flashed);
            if idx_should_flash.is_empty() {
                break;
            } else {
                for idx in idx_should_flash {
                    flashed.insert(idx);
                    self.flashes += 1;
                    let neighbours: Vec<usize> = neighbours(idx as i64, ROW_LENGTH);
                    self.inc_indexes(&neighbours);
                }
            }
        }
        self.reset_indexes(flashed);
    }

    fn step(&mut self) {
        self.inc_all();
        self.flash_all();
    }

    fn simulate_n(&mut self, n: usize) {
        let mut step = 1;

        println!("STEP 0 \n{:}", self);
        while step <= n {
            self.step();
            println!("STEP {:}\n{:}", step, self);
            step += 1;
        }
    }
    fn simulate_until_sync(&mut self) -> usize {
        let mut steps = 0;
        loop {
            let old = self.flashes;
            self.step();
            let new = self.flashes;
            if new == old + self.data.len() {
                break;
            }
            steps += 1;
        }
        steps
    }
}

impl<'a> From<&'a str> for Grid {
    fn from(str: &'a str) -> Self {
        let data = str
            .lines()
            .flat_map(|x| {
                x.chars()
                    .map(|c| c.to_digit(10).expect("valid number") as u8)
            })
            .collect();
        Self { data, flashes: 0 }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for chunk in self.data.chunks_exact(ROW_LENGTH as usize) {
            writeln!(f, "{:?}", chunk).expect("valid");
        }
        write!(f, "")
    }
}

const DIRECTIONS: &[(i64, i64)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn neighbours(point: i64, row_length: i64) -> Vec<usize> {
    let (x, y) = (point % row_length, point / row_length);
    let neighbours = DIRECTIONS
        .iter()
        .map(|(dx, dy)| (x as i64 + dx, y as i64 + dy))
        .filter(|(x, y)| {
            *x >= 0 && *y >= 0 && *x < ROW_LENGTH && (x + y * ROW_LENGTH) < row_length * row_length
        })
        .map(|(x, y)| (x + y * ROW_LENGTH) as usize)
        .collect();
    neighbours
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let mut part1 = Grid::from(input);
    part1.simulate_n(100);

    println!("Part 1: {}", part1.flashes);

    let mut part2 = Grid::from(input);
    let step = part2.simulate_until_sync();
    println!("Part 2: {}", step + 1);
}

mod tests {
    use super::{neighbours, ROW_LENGTH};

    #[test]
    fn neighbours_works() {
        let cases: &[(i64, &[usize])] = &[
            (0, &[10, 1, 11]),
            (12, &[22, 13, 2, 11, 23, 1, 21, 3]),
            (90, &[91, 80, 81]),
            (99, &[89, 98, 88]),
        ];

        for (idx, expected) in cases {
            assert_eq!(*expected, neighbours(*idx, ROW_LENGTH));
        }
    }
}
