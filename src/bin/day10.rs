#[derive(Debug)]
struct Chunks<'a> {
    data: &'a str,
    error: Option<ChunksError>,
}

#[derive(Debug)]
enum ChunksError {
    Incomplete { stack: Vec<char> },
    Invalid { illegal_char: char, pos: usize },
}

fn is_closing_bracket(char: char) -> bool {
    matches!(char, ')' | ']' | '}' | '>')
}

fn bracket_score_invalid(char: char) -> u32 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("unknown score for char {:}", c),
    }
}

fn bracket_score_completion(char: char) -> usize {
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        c => panic!("unknown score for char {:}", c),
    }
}

fn closed_match(char: char) -> char {
    match char {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("unexpected char {:}", char),
    }
}

fn open_match(char: &char) -> char {
    match char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected char {:}", char),
    }
}

impl<'a> Chunks<'a> {
    fn validate(str: &'a str) -> Option<ChunksError> {
        let mut stack: Vec<char> = Vec::new();
        for (idx, c) in str.chars().enumerate() {
            if is_closing_bracket(c) {
                if let Some(last) = stack.last() {
                    let open_bracket = closed_match(c);
                    if open_bracket == *last {
                        stack.pop();
                        continue;
                    }
                }
                return Some(ChunksError::Invalid {
                    illegal_char: c,
                    pos: idx,
                });
            }
            stack.push(c)
        }
        if stack.is_empty() {
            None
        } else {
            Some(ChunksError::Incomplete { stack })
        }
    }
    fn complete_chunks(&self) -> Vec<char> {
        match &self.error {
            Some(ChunksError::Incomplete { stack }) => stack.iter().rev().map(open_match).collect(),

            _ => Vec::new(),
        }
    }
}

impl<'a> From<&'a str> for Chunks<'a> {
    fn from(str: &'a str) -> Self {
        let error = Self::validate(str);
        Self { data: str, error }
    }
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let chunks: Vec<Chunks> = input.lines().map(Chunks::from).collect();

    let part1: u32 = chunks
        .iter()
        .filter_map(|chunk| match chunk.error {
            Some(ChunksError::Invalid { illegal_char, .. }) => {
                Some(bracket_score_invalid(illegal_char))
            }
            _ => None,
        })
        .sum();
    println!("Part 1: {:}", part1);

    let mut part2_candidates: Vec<usize> = chunks
        .iter()
        .map(Chunks::complete_chunks)
        .flat_map(|completed| {
            completed
                .into_iter()
                .map(bracket_score_completion)
                .reduce(|accum, item| accum * 5 + item)
        })
        .collect();
    part2_candidates.sort_unstable();
    println!("Part 2: {:}", part2_candidates[part2_candidates.len() / 2]);
}
