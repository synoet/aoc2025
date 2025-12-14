enum Direction {
    Left,
    Right,
}

fn wrapping_add(a: u32, b: u32, max: u32) -> u32 {
    (a + b) % (max)
}

fn wrapping_sub(a: u32, b: u32, max: u32) -> u32 {
    let b_mod = b % max;
    if a >= b_mod {
        a - b_mod
    } else {
        max - (b_mod - a)
    }
}

fn rotate(dial: u32, direction: &Direction, num: u32) -> u32 {
    match direction {
        Direction::Left => wrapping_sub(dial, num, 100),
        Direction::Right => wrapping_add(dial, num, 100),
    }
}

fn count_clicks_exact(dial: u32, _num: u32, _direction: &Direction) -> u32 {
    if dial == 0 { 1 } else { 0 }
}

fn count_zero_crossings(dial: u32, num: u32, direction: &Direction) -> u32 {
    match direction {
        Direction::Right => (dial + num) / 100,
        Direction::Left => {
            if dial == 0 {
                num / 100
            } else if num >= dial {
                1 + (num - dial) / 100
            } else {
                0
            }
        }
    }
}

mod input {
    use super::*;
    pub const TEST_CASE: &str = r#"
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    "#;

    pub fn parse_input(input: &str) -> Vec<(Direction, u32)> {
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                let direction = match line.chars().next().unwrap() {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Invalid direction"),
                };

                let num = line
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                (direction, num)
            })
            .collect()
    }
}

#[cfg(feature = "part_one")]
mod part_one {
    use super::*;

    pub fn run(input: &str, initial_dial: u32) -> u32 {
        input::parse_input(input)
            .into_iter()
            .fold((initial_dial, 0), |(dial, hits), (direction, num)| {
                let new_dial = rotate(dial, &direction, num);
                let count = count_clicks_exact(dial, num, &direction);
                (new_dial, hits + count)
            })
            .1
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_case() {
            assert_eq!(run(input::TEST_CASE, 50), 3);
        }
    }
}

#[cfg(feature = "part_two")]
mod part_two {
    use super::*;

    pub fn run(input: &str, initial_dial: u32) -> u32 {
        input::parse_input(input)
            .into_iter()
            .fold((initial_dial, 0), |(dial, hits), (direction, num)| {
                let crossings = count_zero_crossings(dial, num, &direction);
                let new_dial = rotate(dial, &direction, num);
                (new_dial, hits + crossings)
            })
            .1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_case() {
            assert_eq!(run(input::TEST_CASE, 50), 6);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    if cfg!(feature = "part_one") {
        let out = part_one::run(input, 50);
        println!("Part 1: {}", out);
    }

    if cfg!(feature = "part_two") {
        let out = part_two::run(input, 50);
        println!("Part 2: {}", out);
    }
}
