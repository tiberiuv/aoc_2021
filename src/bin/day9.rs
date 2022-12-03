use std::collections::HashSet;

struct Grid {
    data: Vec<u8>,
    row_width: usize,
    low_points: HashSet<usize>,
}

fn apply(x: i32, y: i32, op: (i32, i32)) -> (i32, i32) {
    (op.0 + x, op.1 + y)
}

impl Grid {
    const TOP: (i32, i32) = (0, -1);
    const BOTTOM: (i32, i32) = (0, 1);
    const RIGHT: (i32, i32) = (1, 0);
    const LEFT: (i32, i32) = (-1, 0);
    const POSITIONS: &'static [(i32, i32)] = &[Self::TOP, Self::BOTTOM, Self::RIGHT, Self::LEFT];

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<usize> {
        Self::POSITIONS
            .iter()
            .map(|op| apply(x as i32, y as i32, *op))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && *y < ((self.data.len() / self.row_width) as i32)
                    && x + (y * self.row_width as i32) < self.data.len() as i32
            })
            .map(|(x, y)| x as usize + (y as usize * self.row_width))
            .collect()
    }

    fn mark_lowest(&mut self, locations: &[(usize, u8)], current_idx: usize) {
        if let Some((found_idx, _)) = locations.iter().min_by_key(|x| x.1) {
            if *found_idx == current_idx {
                self.low_points.insert(current_idx);
            }
        }
    }

    fn part_a(&mut self) -> String {
        for idx in 0..self.data.len() {
            let x = idx % self.row_width;
            let y = idx / self.row_width;

            let mut positions = self.get_adjacent(x, y);
            // add the current point
            positions.push(idx);

            let items: Vec<(usize, u8)> = positions
                .into_iter()
                .map(|idx| (idx, self.data[idx]))
                .collect();
            self.mark_lowest(&items, idx)
        }

        let sum: usize = self
            .low_points
            .iter()
            .map(|pos| (self.data[*pos] + 1) as usize)
            .sum();
        format!("Part A | {}", sum)
    }

    fn part_b(&mut self) -> String {
        self.part_a();
        let product: usize = self
            .low_points
            .iter()
            .map(|x| self.find_basin(*x))
            .product();
        format!("Part B | {}", product)
    }

    fn find_basin(&self, idx: usize) -> usize {
        let x = idx % self.row_width;
        let y = idx / self.row_width;

        let value = self.data[idx];
        let positions = self.get_adjacent(x, y);

        if positions.is_empty() {
            return 1;
        }

        let result = positions
            .iter()
            .map(|idx| (*idx, self.data[*idx]))
            .filter(|(_, v)| *v > value && *v != 9)
            .map(|(idx, _)| self.find_basin(idx))
            .sum();
        result
    }
}

fn main() {
    let mut input = include_str!("../inputs/day9.txt").lines().peekable();
    let row_width = input.peek().map(|line| line.len()).expect("Invalid input");
    let input: Vec<u8> = input
        .flat_map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).expect("Not a digit!") as u8)
        })
        .collect();

    let mut grid = Grid {
        data: input,
        row_width,
        low_points: HashSet::new(),
    };

    // println!("{}", grid.part_a());
    println!("{}", grid.part_b());
}
