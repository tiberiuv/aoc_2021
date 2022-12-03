use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    X(usize),
    Y(usize),
}

impl<'a> From<&'a str> for Fold {
    fn from(str: &'a str) -> Self {
        let axis = str.chars().nth(11).unwrap_or_default();
        let number = str
            .chars()
            .skip(13)
            .collect::<String>()
            .parse()
            .unwrap_or_default();
        match axis {
            'x' => Self::X(number),
            'y' => Self::Y(number),
            _ => panic!("Bad input!"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point(usize, usize);
impl Point {
    fn try_apply_fold(&self, fold: Fold) -> Option<Point> {
        let Point(x, y) = self;
        match fold {
            Fold::X(fold_x) if *x > fold_x => Some(Point(fold_x * 2 - x, *y)),
            Fold::Y(fold_y) if *y > fold_y => Some(Point(*x, fold_y * 2 - y)),
            _ => None,
        }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

#[derive(Debug)]
struct Grid {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

impl Grid {
    fn print_points(&self) {
        let (max_x, max_y) = self.get_max_xy();
        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                if self.points.contains(&Point(x, y)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
    fn apply_folds(&mut self) -> Vec<usize> {
        let mut len_points = Vec::with_capacity(8);
        for instruction in self.folds.clone() {
            self.apply_single_fold(instruction);
            len_points.push(self.points.len());
        }
        len_points
    }
    fn apply_single_fold(&mut self, fold: Fold) {
        let mut new_points = self.points.clone();
        for point in &self.points {
            if let Some(new_point) = point.try_apply_fold(fold) {
                new_points.insert(new_point);
                new_points.remove(point);
            }
        }
        self.points = new_points;
    }
    fn get_max_xy(&self) -> (usize, usize) {
        self.points
            .iter()
            .fold((0, 0), |(max_x, max_y), Point(x, y)| {
                let new_x = if &max_x < x { *x } else { max_x };
                let new_y = if &max_y < y { *y } else { max_y };
                (new_x, new_y)
            })
    }
}

impl<'a> From<&'a str> for Grid {
    fn from(str: &'a str) -> Self {
        let (points, folds) = str.split_at(str.find("\n\n").expect("invalid input"));
        let points: HashSet<Point> = points
            .lines()
            .map(|x| {
                let (x, y) = x.split_at(x.find(',').expect("invalid input"));
                let x = x.parse().expect("NAN");
                let y = y.parse().expect("NAN");
                Point(x, y)
            })
            .collect();
        let folds: Vec<Fold> = folds.lines().map(Fold::from).collect();

        Grid { points, folds }
    }
}

fn main() {
    let input = include_str!("../inputs/day13.txt");
    let mut grid = Grid::from(input);
    let len_points = grid.apply_folds();

    println!("Part 1 {}", len_points[0]);
    println!("Part 2 \n");
    grid.print_points()
}
