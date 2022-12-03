use std::collections::HashMap;

fn main() {
    let mapping = HashMap::from([(2, 1), (4, 4), (3, 7), (7, 8)]);
    let input: Vec<(Vec<&str>, Vec<&str>)> = include_str!("../inputs/day8.txt")
        .lines()
        .map(|x| {
            let (input, output) = x.split_once(" | ").unwrap();
            let input_items: Vec<&str> = input.split(' ').collect();
            let output_items: Vec<&str> = output.split(' ').collect();

            (input_items, output_items)
        })
        .collect();

    let outputs: Vec<&str> = input.iter().flat_map(|x| x.1.clone()).collect();

    let mut occurences = 0;
    for item in &outputs {
        let len_item = item.len() as u8;
        if mapping.get(&len_item).is_some() {
            occurences += 1
        }
    }

    println!("Part 1 {}", occurences);
}

fn part2_find_full_mapping<'a>(inputs: &'a [&'a str]) -> HashMap<&'a str, u8> {
    let mapping = HashMap::from([(2, 1), (4, 4), (3, 7), (7, 8)]);
    let mut digit_to_word: HashMap<u8, &'a str> = inputs
        .iter()
        .filter_map(|word| mapping.get(&word.len()).map(|digit| (*digit, *word)))
        .collect();

    let word_to_digit = HashMap::new();
    let sixes = inputs.iter().filter(|n| n.len() == 6);
    let nines = inputs.iter().filter(|n| n.len() == 9);

    for &word in sixes.clone() {
        if word
            .find(|x| !digit_to_word.get(&1).unwrap().contains(x))
            .is_some()
        {
            digit_to_word.insert(6, word);
        }
    }
    for word in sixes {
        if word
            .find(|x| !digit_to_word.get(&4).unwrap().contains(x))
            .is_some()
        {
            digit_to_word.insert(6, word);
        }
    }
    word_to_digit
}

fn part_2(inputs: Vec<&str>, outputs: Vec<&str>) {}
