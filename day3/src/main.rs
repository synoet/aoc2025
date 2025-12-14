static TEST_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111
";

fn parse_input(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|s| {
            dbg!(s.to_string());
            s.to_string()
                .chars()
                .map(|s| s.to_string().parse().expect("should be parsed as number"))
                .collect()
        })
        .collect::<Vec<Vec<u16>>>()
}

fn find_highest_with_index(nums: &[u16]) -> (u16, usize) {
    let mut max: u16 = 0;
    let mut index = 0;
    for (i, n) in nums.iter().enumerate() {
        if *n > max {
            max = *n;
            index = i;
        }
    }
    (max, index)
}

fn part_one() -> u64 {
    let mut total = 0;
    let banks = parse_input(include_str!("../input.txt"));

    for bank in banks {
        let (first, fi) = find_highest_with_index(&bank[..bank.len() - 1]);
        let (second, _) = find_highest_with_index(&bank[fi + 1..]);

        let val: u64 = format!("{}{}", first, second)
            .parse()
            .expect("parse as int");
        total = total + val;
    }

    total
}

const MAX_BANKS: u64 = 12;

fn part_two() -> u64 {
    let banks = parse_input(include_str!("../input.txt"));

    let mut total: u64 = 0;
    for bank in banks {
        let mut joltage = String::new();
        let mut start_index = 0;
        for i in 0..MAX_BANKS {
            let needed = MAX_BANKS - i;

            let (f, fi) =
                find_highest_with_index(&bank[start_index..(bank.len() + 1) - needed as usize]);

            joltage.push_str(&f.to_string());
            start_index = start_index + fi + 1;
        }
        total += joltage.parse::<u64>().expect("can parse as string");
    }

    total
}

fn main() {
    // part_one();
    part_two();
}
