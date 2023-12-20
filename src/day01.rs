fn main() {
    let input = include_str!("day01.txt");

    println!("part1: {}", part1::solve(input));
    println!("part2: {}", part2::solve(input));
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
        let mut it = input.chars().filter_map(|c| c.to_digit(10));
        let first = it.next().unwrap();
        let last = it.last().unwrap_or(first);
        (first * 10 + last) as i32
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
        fn test_solve_part1() {
            let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
            assert_eq!(solve(input), 142);
        }
    }
}

mod part2 {
    pub fn solve_line(input: &str) -> i32 {
        const REPLACE_FROM: &[&str] = &[
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        const REPLACE_TO: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

        let mut output = String::new();
        for (idx, _c) in input.char_indices() {
            for i in 0..REPLACE_FROM.len() {
                if Some(0) == input[idx..].find(REPLACE_FROM[i]) {
                    output.push_str(REPLACE_TO[i]);
                    break;
                }

                if Some(0) == input[idx..].find(REPLACE_TO[i]) {
                    output.push_str(REPLACE_TO[i]);
                    break;
                }
            }
        }
        crate::part1::solve_line(&output)
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
        fn test_solve() {
            let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
            assert_eq!(solve(input), 281);
        }
    }
}
