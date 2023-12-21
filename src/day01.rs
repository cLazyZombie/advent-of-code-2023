fn main() {
    let input = include_str!("day01.txt");

    println!("part1: {}", part1::solve(input));
    println!("part2: {}", part2::solve(input));
}

pub fn solve_line_with_mapping(input: &str, mapping: &[(&str, i32)]) -> i32 {
    // tuple (found_index_in_input, index_in_mapping)
    let mut left = None;
    for (idx, &(s, _)) in mapping.iter().enumerate() {
        let found = input.find(s);
        match (left, found) {
            (None, Some(found)) => left = Some((found, idx)),
            (Some((prev, _)), Some(found)) if found < prev => left = Some((found, idx)),
            _ => {}
        }
    }

    let mut right = None;
    for (idx, &(s, _)) in mapping.iter().enumerate() {
        let found = input.rfind(s);
        match (right, found) {
            (None, Some(found)) => right = Some((found, idx)),
            (Some((prev, _)), Some(found)) if found > prev => right = Some((found, idx)),
            _ => {}
        }
    }

    mapping[left.unwrap().1].1 * 10 + mapping[right.unwrap().1].1
}
mod part1 {
    pub fn solve(input: &str) -> i32 {
        let mut sum = 0;
        for line in input.lines() {
            sum += solve_line(line.trim());
        }
        sum
    }

    pub fn solve_line(input: &str) -> i32 {
        const MAPPING: &[(&str, i32)] = &[
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ];

        crate::solve_line_with_mapping(input, MAPPING)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve_line() {
            assert_eq!(solve_line("1abc2"), 12);
            assert_eq!(solve_line("pqr3stu8vwx"), 38);
            assert_eq!(solve_line("a1b2c3d4e5f"), 15);
            assert_eq!(solve_line("treb7uchet"), 77);
        }

        #[test]
        fn test_solve_sample() {
            let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
            assert_eq!(solve(input), 142);
        }

        #[test]
        fn test_solve() {
            let input = include_str!("day01.txt");
            assert_eq!(solve(input), 53651);
        }
    }
}

mod part2 {
    pub fn solve_line(input: &str) -> i32 {
        const MAPPING: &[(&str, i32)] = &[
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        crate::solve_line_with_mapping(input, MAPPING)
    }

    pub fn solve(input: &str) -> i32 {
        let mut sum = 0;
        for line in input.lines() {
            sum += solve_line(line.trim());
        }
        sum
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve_line() {
            assert_eq!(solve_line("two1nine"), 29);
            assert_eq!(solve_line("eightwothree"), 83);
            assert_eq!(solve_line("abcone2threexyz"), 13);
            assert_eq!(solve_line("xtwone3four"), 24);
            assert_eq!(solve_line("4nineeightseven2"), 42);
            assert_eq!(solve_line("zoneight234"), 14);
            assert_eq!(solve_line("7pqrstsixteen"), 76);
        }

        #[test]
        fn test_edge_case() {
            assert_eq!(solve_line("threeeighthree"), 33);
            assert_eq!(solve_line("eight8zlctbmsixhrvbpjb84nnmlcqkzrsix"), 86);
            assert_eq!(
                solve_line("seven8sevenptdlvvgssixvjvzpvsp7fivefourtwoned"),
                71
            );
        }

        #[test]
        fn test_solve_sample() {
            let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
            assert_eq!(solve(input), 281);
        }

        #[test]
        fn test_solve() {
            let input = include_str!("day01.txt");
            assert_eq!(solve(input), 53894);
        }
    }
}
