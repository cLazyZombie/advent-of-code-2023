fn main() {
    let input = include_str!("./day03.txt");
    println!("day03 part1: {}", part1::solve(input));
    println!("day03 part2: {}", part2::solve(input));
}

struct Engine {
    pub lines: Vec<Vec<char>>,
}
impl Engine {
    pub fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        Self { lines }
    }

    pub fn width(&self) -> i32 {
        self.lines[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.lines.len() as i32
    }

    pub fn is_symbol(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            return false;
        }

        let c = self.lines[y as usize][x as usize];
        if c.is_numeric() || c == '.' {
            false
        } else {
            true
        }
    }

    pub fn is_gear(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            return false;
        }

        let c = self.lines[y as usize][x as usize];
        if c == '*' {
            true
        } else {
            false
        }
    }

    pub fn get_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        let mut num = Option::<Number>::None;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let c = self.lines[y as usize][x as usize];
                if c.is_numeric() {
                    if let Some(n) = &mut num {
                        n.value = n.value * 10 + c.to_digit(10).unwrap() as i32;
                        n.end_x = x;
                    } else {
                        num = Some(Number {
                            value: c.to_digit(10).unwrap() as i32,
                            y,
                            start_x: x,
                            end_x: x,
                        });
                    }
                } else {
                    if let Some(n) = num {
                        numbers.push(n);
                        num = None;
                    }
                }
            }

            // save prev number is new line
            if let Some(n) = num {
                numbers.push(n);
                num = None;
            }
        }

        numbers
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Number {
    pub value: i32,
    pub y: i32,
    pub start_x: i32,
    pub end_x: i32,
}
impl Number {
    pub fn is_adjacent(&self, x: i32, y: i32) -> bool {
        if y > self.y + 1 || y < self.y - 1 {
            return false;
        }

        if x < self.start_x - 1 || x > self.end_x + 1 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_engine() {
        let input_str = r#"..#...
....@$"#;
        let engine = Engine::new(input_str);
        assert_eq!(engine.width(), 6);
        assert_eq!(engine.height(), 2);
    }

    #[test]
    fn test_get_numbers() {
        let input_str = r#"1#.23
        .4$56"#;
        let engine = Engine::new(input_str);
        let numbers = engine.get_numbers();
        assert_eq!(numbers.len(), 4);
        assert_eq!(
            numbers[0],
            Number {
                value: 1,
                y: 0,
                start_x: 0,
                end_x: 0
            }
        );
        assert_eq!(
            numbers[1],
            Number {
                value: 23,
                y: 0,
                start_x: 3,
                end_x: 4
            }
        );
        assert_eq!(
            numbers[2],
            Number {
                value: 4,
                y: 1,
                start_x: 1,
                end_x: 1
            }
        );
        assert_eq!(
            numbers[3],
            Number {
                value: 56,
                y: 1,
                start_x: 3,
                end_x: 4
            }
        );
    }
}

mod part1 {
    use super::*;

    pub fn solve(input: &str) -> i32 {
        let engine = Engine::new(input);
        let numbers = engine.get_numbers();
        let sum = numbers
            .iter()
            .filter(|n| {
                for x in (n.start_x - 1)..=(n.end_x + 1) {
                    if engine.is_symbol(x, n.y - 1) || engine.is_symbol(x, n.y + 1) {
                        return true;
                    }
                }

                if engine.is_symbol(n.start_x - 1, n.y) || engine.is_symbol(n.end_x + 1, n.y) {
                    return true;
                }

                false
            })
            .map(|n| n.value)
            .sum();
        sum
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let input_str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
            assert_eq!(solve(input_str), 4361);
        }

        #[test]
        fn test_solve() {
            let input_str = include_str!("./day03.txt");
            assert_eq!(solve(input_str), 532428);
        }
    }
}

mod part2 {
    use super::*;

    pub fn solve(input: &str) -> i32 {
        let engine = Engine::new(input);
        let numbers = engine.get_numbers();
        let mut sum = 0;
        for y in 0..engine.height() {
            for x in 0..engine.width() {
                if engine.is_gear(x, y) {
                    let gear_numbers = numbers
                        .iter()
                        .filter(|n| n.is_adjacent(x, y))
                        .collect::<Vec<_>>();

                    if gear_numbers.len() == 2 {
                        sum += gear_numbers[0].value * gear_numbers[1].value;
                    }
                }
            }
        }
        sum
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let input_str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
            assert_eq!(solve(input_str), 467835);
        }

        #[test]
        fn test_solve() {
            let input_str = include_str!("./day03.txt");
            assert_eq!(solve(input_str), 84051670);
        }
    }
}
