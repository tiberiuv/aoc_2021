fn calc_population(days: usize, initial: &[usize]) -> usize {
    let mut ages = [0; 9];
    initial.iter().for_each(|x| ages[*x] += 1);

    for _ in 0..days {
        let new = ages[0];
        for x in 0..8 {
            ages[x] = ages[x + 1];
        }
        ages[6] += new;
        ages[8] = new;
    }
    ages.iter().sum()
}

fn main() {
    let input: Vec<usize> = include_str!("../inputs/day6.txt")
        .lines()
        .next()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or_default();

    let a = calc_population(80, &input);
    let b = calc_population(256, &input);

    println!("Part a: {}", a);
    println!("Part b: {}", b);
}
