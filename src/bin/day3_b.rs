fn nth_bit_set(x: u16, n: usize) -> bool {
    x & (1 << n) > 0
}

fn find_num(list: Vec<u16>, most_common: bool, idx: usize) -> u16 {
    if list.len() == 1 {
        return list[0];
    }
    let count_nth_bit = list
        .iter()
        .fold(0, |accum, x| accum + nth_bit_set(*x, idx) as u16);

    let find = if most_common {
        count_nth_bit >= (list.len() as u16 - count_nth_bit)
    } else {
        count_nth_bit < (list.len() as u16 - count_nth_bit)
    };

    let new_list: Vec<u16> = list
        .into_iter()
        .filter(|x| nth_bit_set(*x, idx) == find)
        .collect();

    let next_idx = if idx > 1 { idx - 1 } else { 0 };
    find_num(new_list, most_common, next_idx)
}

fn main() {
    let input = include_str!("../inputs/day3.txt").lines();
    let digits = input.clone().next().map(|x| x.len()).unwrap_or(1) - 1;
    let list: Vec<u16> = input
        .map(|x| isize::from_str_radix(x, 2).unwrap() as u16)
        .collect();

    let first = find_num(list.clone(), true, digits);
    let second = find_num(list, false, digits);
    println!("{}", (first as usize * second as usize));
}
