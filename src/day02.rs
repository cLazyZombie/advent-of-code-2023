fn main() {
    let input = include_str!("day02.txt");

    println!("part1: {}", part1::solve(input));
    println!("part2: {}", part2::solve(input));
}

struct Game {
    pub id: i32,
    pub cubes: Vec<Cube>,
}

#[derive(Debug, Eq, PartialEq)]
struct Cube {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

fn parse_game(input: &str) -> Game {
    let (game_str, cubes_str) = input.split_once(":").unwrap();

    let game_id = game_str
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut cubes = Vec::new();
    for cube_str in cubes_str.split(";") {
        let cube = parse_cube(cube_str);
        cubes.push(cube);
    }

    Game { id: game_id, cubes }
}

fn parse_cube(input: &str) -> Cube {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    let cubes = input.split(",").map(|s| s.trim());
    for c in cubes {
        let mut w = c.split_whitespace();
        let count = w.next().unwrap();
        let color = w.next().unwrap();
        match color {
            "red" => red += count.parse::<i32>().unwrap(),
            "blue" => blue += count.parse::<i32>().unwrap(),
            "green" => green += count.parse::<i32>().unwrap(),
            _ => panic!("unknown color [{}]", color),
        }
    }

    Cube { red, blue, green }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 1 red, 1 blue; 2 red, 2 green; 10 red, 100 green, 1000 blue";
        let game = parse_game(input);
        assert_eq!(game.id, 1);
        assert_eq!(game.cubes.len(), 3);
        assert_eq!(
            game.cubes[0],
            Cube {
                red: 1,
                green: 0,
                blue: 1,
            }
        );
        assert_eq!(
            game.cubes[1],
            Cube {
                red: 2,
                green: 2,
                blue: 0,
            }
        );
        assert_eq!(
            game.cubes[2],
            Cube {
                red: 10,
                green: 100,
                blue: 1000,
            }
        );
    }
}

mod part1 {
    use crate::Game;

    fn is_possible(game: &Game) -> bool {
        for cube in &game.cubes {
            if cube.red > 12 || cube.green > 13 || cube.blue > 14 {
                return false;
            }
        }

        true
    }

    pub fn solve(input: &str) -> i32 {
        let mut games = Vec::new();
        for line in input.lines() {
            let game = crate::parse_game(line);
            games.push(game);
        }

        games
            .iter()
            .map(|game| {
                if is_possible(game) {
                    Some(game.id)
                } else {
                    None
                }
            })
            .filter_map(|id| id)
            .fold(0, |acc, id| acc + id)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve() {
            let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

            assert_eq!(solve(input), 8);
        }
    }
}

mod part2 {
    use crate::Game;

    pub fn solve_game(game: &Game) -> i32 {
        let red = game.cubes.iter().max_by_key(|c| c.red).unwrap().red;
        let green = game.cubes.iter().max_by_key(|c| c.green).unwrap().green;
        let blue = game.cubes.iter().max_by_key(|c| c.blue).unwrap().blue;

        red * green * blue
    }

    pub fn solve(input: &str) -> i32 {
        let mut games = Vec::new();
        for line in input.lines() {
            let game = crate::parse_game(line);
            games.push(game);
        }

        games
            .iter()
            .map(|game| solve_game(game))
            .fold(0, |acc, min| acc + min)
    }

    #[cfg(test)]
    mod test {
        use crate::parse_game;

        use super::*;

        #[test]
        fn test_solve_game() {
            let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
            let game = parse_game(input);
            assert_eq!(solve_game(&game), 48);
        }

        #[test]
        fn test_solve_sample() {
            let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
            assert_eq!(solve(input), 2286);
        }
    }
}
