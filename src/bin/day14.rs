use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Rule<'a> {
    pair: &'a str,
    atom: char,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(str: &'a str) -> Self {
        let mut split = str.splitn(2, " -> ");

        let pair = split.next().unwrap_or_default();
        let atom = split
            .next()
            .unwrap_or_default()
            .chars()
            .next()
            .expect("invalid input");

        Self { pair, atom }
    }
}

#[derive(Debug)]
struct Polymer<'a> {
    template: String,
    rules: HashMap<&'a str, char>,
    counts: HashMap<char, u32>,
}

fn insert_char(str: &str, char: char) -> String {
    let mut string = str.to_string();

    string.insert(1, char);
    string
}

impl<'a> Polymer<'a> {
    fn apply_rules(&mut self, steps: u8) {
        for idx in 0..steps {
            println!("Iteration {}", idx + 1);
            self.apply_rules_single();
        }
    }

    fn apply_rules_single(&mut self) {
        let mut parts = Vec::new();
        let mut last_matched = false;
        for idx in 0..self.template.len() - 1 {
            let part = self.template.get(idx..idx + 2).unwrap();
            if let Some(&atom) = self.rules.get(part) {
                if last_matched {
                    let p = &part[1..2];
                    let mut s = String::from(p);
                    s.insert(0, atom);
                    parts.push(s);
                } else {
                    parts.push(insert_char(part, atom));
                }
                last_matched = true;
            } else {
                parts.push(part.to_string());
                last_matched = false;
            }
        }
        self.template = parts.join("");
    }

    fn find_min_max(&self) -> ((char, i32), (char, i32)) {
        let mut counts = HashMap::new();
        let mut max = ('_', 0);
        let mut min = ('_', i32::MAX);
        for char in self.template.chars() {
            counts.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }

        for (c, count) in counts {
            if count > max.1 {
                max = (c, count)
            }
            if count < min.1 {
                min = (c, count)
            }
        }

        (min, max)
    }
}

impl<'a> From<&'a str> for Polymer<'a> {
    fn from(str: &'a str) -> Self {
        let mut split = str.splitn(2, "\n\n");
        let template = split.next().unwrap_or_default().to_string();
        let rules = split
            .next()
            .unwrap_or_default()
            .lines()
            .map(Rule::from)
            .map(|x| (x.pair, x.atom))
            .collect();

        let mut counts = HashMap::new();

        for char in template.chars() {
            counts.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }

        Self {
            template,
            rules,
            counts,
        }
    }
}

fn main() {
    let mut polymer = Polymer::from(include_str!("../inputs/day14.txt"));

    polymer.apply_rules(10);
    let (min, max) = polymer.find_min_max();
    println!("Part 1 {}", max.1 - min.1);
}
