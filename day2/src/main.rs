static TEST_INPUT: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"#;

type Range = (u64, u64);

fn parse_input_into_range(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|l| {
            l.trim()
                .split_once('-')
                .expect("line should only have a single '-'")
        })
        .map(|r| {
            (
                r.0.parse().expect("start range to be parsable"),
                r.1.parse().expect("end range to be parsable"),
            )
        })
        .collect::<Vec<Range>>()
}

fn expand_range(range: &Range) -> Vec<u64> {
    let diff = range.1 - range.0;
    assert!(diff > 0);
    let mut result: Vec<u64> = vec![];
    for i in 0..(diff + 1) {
        result.push(range.0 + i)
    }

    result
}

fn is_repeating_twice_invalid(id: u64) -> bool {
    let str_id = id.to_string();
    let length = str_id.len();
    if length % 2 != 0 {
        return false;
    }
    let (l, r) = str_id.split_at(length / 2);

    if l == r {
        return true;
    }

    false
}

fn is_pattern_repeating(pattern: &str, set: &str) -> bool {
    let pattern_length = pattern.len();
    let set_length = set.len();

    if set_length % pattern_length != 0 {
        return false;
    }

    set.chars()
        .collect::<Vec<_>>()
        .chunks(pattern_length)
        .all(|c| c.iter().collect::<String>() == pattern)
}

fn is_any_part_repeating(id: u64) -> bool {
    let id_str = id.to_string();

    let max_window_length = (id_str.len() / 2) + 1;

    for window_length in 1..max_window_length {
        if is_pattern_repeating(&id_str[0..window_length], &id_str) {
            return true;
        }
    }

    false
}

fn part_one() -> u64 {
    let input = include_str!("../input.txt");
    let ids: Vec<u64> = parse_input_into_range(input)
        .iter()
        .map(expand_range)
        .flatten()
        .collect();

    ids.into_iter()
        .filter(|id| is_repeating_twice_invalid(*id).then_some(id).is_some())
        .reduce(|acc, e| acc + e)
        .unwrap_or(0)
}

fn part_two() -> u64 {
    let input = include_str!("../input.txt");
    let ids: Vec<u64> = parse_input_into_range(input)
        .iter()
        .map(expand_range)
        .flatten()
        .collect();

    ids.into_iter()
        .filter(|id| is_any_part_repeating(*id).then_some(id).is_some())
        .reduce(|acc, e| acc + e)
        .unwrap_or(0)
}

fn main() {
    // part_one();
    part_two();
}
