use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Board {
    // the numbers on the board
    items: Vec<usize>,
    items_map: HashMap<usize, usize>,
    // list of indexes of numbers which we have seen
    seen: HashSet<usize>,
}

impl Board {
    fn is_bingo(&self) -> bool {
        self.is_cols_bingo() || self.is_rows_bingo()
    }

    fn is_cols_bingo(&self) -> bool {
        let cols = get_col_indexes();
        cols.iter().any(|x| self.seen.is_superset(x))
    }

    fn is_rows_bingo(&self) -> bool {
        let rows = get_row_indexes();
        rows.iter().any(|x| self.seen.is_superset(x))
    }

    fn get_unseen(&self) -> impl Iterator<Item = &usize> {
        self.items
            .iter()
            .enumerate()
            .filter(|(idx, _)| !self.seen.contains(idx))
            .map(|(_, x)| x)
    }

    fn insert_seen(&mut self, idx: usize) {
        self.seen.insert(idx);
    }
}

fn get_col_indexes() -> Vec<HashSet<usize>> {
    (0..5_usize)
        .map(|x| (0..5_usize).map(|y| y * 5 + x).collect::<HashSet<usize>>())
        .collect()
}

fn get_row_indexes() -> Vec<HashSet<usize>> {
    (0..5_usize)
        .map(|x| (x * 5..x * 5 + 5).collect::<HashSet<usize>>())
        .collect()
}

fn main() {
    let mut input = include_str!("../inputs/day4.txt").lines();
    // parse first line - the random numbers
    let numbers: Vec<usize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // skip empty line
    input.next();
    let input: Vec<&str> = input.collect();
    let mut boards: Vec<Board> = Vec::with_capacity(2 ^ 5);

    for lines in input.chunks(6) {
        let board_numbers = lines
            .iter()
            .flat_map(|line| line.split(' ').flat_map(|n| n.parse::<usize>()))
            .collect::<Vec<usize>>();

        let board = Board {
            items: board_numbers.clone(),
            items_map: board_numbers
                .into_iter()
                .enumerate()
                .map(|(idx, n)| (n, idx))
                .collect(),
            seen: HashSet::new(),
        };
        println!("{:?}", board);
        boards.push(board);
    }
    let mut is_bingo = false;
    for n in numbers {
        for b in &mut boards {
            let idx = if let Some(idx) = b.items_map.get(&n) {
                *idx
            } else {
                continue;
            };
            b.insert_seen(idx);

            if b.is_bingo() {
                is_bingo = true;
                let sum: usize = b.get_unseen().sum();

                println!("Bingo! {} {}", sum, n);
                println!("Bingo! {}", sum * n);
                break;
            }
        }
        if is_bingo {
            break;
        }
    }
}
