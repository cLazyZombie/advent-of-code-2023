use std::collections::HashSet;

fn main() {
    let input = include_str!("day04.txt");

    println!("part1: {}", part1::solve(input));
    println!("part2: {}", part2::solve(input));
}

pub struct Card {
    pub id: i32,
    pub your_numbers: HashSet<i32>,
    pub winning_numbers: HashSet<i32>,
}

fn parse_card(input: &str) -> Card {
    let (card_str, numbers_str) = input.split_once(':').unwrap();
    let (winning_numbers_str, your_numbers_str) = numbers_str.split_once('|').unwrap();

    let card_id = card_str
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let winning_numbers = winning_numbers_str
        .split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let your_numbers = your_numbers_str
        .split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    Card {
        id: card_id,
        your_numbers,
        winning_numbers,
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(parse_card).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(input);
        assert_eq!(card.id, 1);

        assert_eq!(card.your_numbers.len(), 8);
        assert_eq!(
            card.your_numbers,
            [83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect()
        );

        assert_eq!(card.winning_numbers.len(), 5);
        assert_eq!(
            card.winning_numbers,
            [41, 48, 83, 86, 17].into_iter().collect()
        );
    }
}

mod part1 {
    use super::*;

    pub fn solve_line(card: &Card) -> i32 {
        let count = card
            .your_numbers
            .intersection(&card.winning_numbers)
            .count() as u32;

        if count == 0 {
            return 0;
        }
        2_i32.pow(count - 1)
    }

    pub fn solve(input: &str) -> i32 {
        let cards = parse_cards(input);
        cards.iter().map(solve_line).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve_line() {
            let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let card = parse_card(input);
            assert_eq!(solve_line(&card), 8);
        }

        #[test]
        fn test_solve_sample() {
            let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
            assert_eq!(solve(input), 13);
        }

        #[test]
        fn test_solve() {
            let input = include_str!("day04.txt");
            assert_eq!(solve(input), 21821);
        }
    }
}

mod part2 {
    use super::*;

    pub fn solve(input: &str) -> i32 {
        let card_matches = parse_cards(input)
            .iter()
            .map(|c| c.your_numbers.intersection(&c.winning_numbers).count())
            .collect::<Vec<_>>();

        let mut card_count = vec![1; card_matches.len()];

        for (card_idx, &match_count) in card_matches.iter().enumerate() {
            let plus = card_count[card_idx] as i32;
            for card in card_count.iter_mut().skip(card_idx + 1).take(match_count) {
                *card += plus;
            }
        }

        card_count.iter().sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
            assert_eq!(solve(input), 30);
        }

        #[test]
        fn test_solve() {
            let input = include_str!("day04.txt");
            assert_eq!(solve(input), 5539496);
        }
    }
}
