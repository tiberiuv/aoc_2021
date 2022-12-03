fn main() {
    let input: Vec<&str> = include_str!("../inputs/day3.txt").lines().collect();

    let input_count = input.len() as i16;

    let num_bits = input.first().unwrap().len();
    let mut count_set_bits: Vec<i16> = vec![0; num_bits];
    for l in input {
        for (idx, c) in l.chars().enumerate() {
            if c == '1' {
                count_set_bits[idx] += 1;
            }
        }
    }
    let mut g = 0;
    let mut e = 0;
    for (idx, i) in count_set_bits.iter().enumerate() {
        let bit = 1 << (num_bits - 1 - idx);
        if i > &(input_count / 2) {
            g |= bit;
        } else {
            e |= bit;
        }
        println!("{} {}", g, e);
    }

    println!("{} {}", g, e);
    println!("{}", g * e);
}
