use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    let winning_regex = Regex::new(r":(.*?)\|").unwrap();
    let player_regex = Regex::new(r"\|(.*?)$").unwrap();
    for card_line in input.split("\n") {
        let mut winners: HashSet<&str> = HashSet::new();
        let winner_nums = winning_regex
            .captures(card_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(" ")
            .filter(|c| !c.is_empty());
        let player_nums: Vec<&str> = player_regex
            .captures(card_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(" ")
            .filter(|c| !c.is_empty())
            .collect();

        for n in winner_nums {
            winners.insert(n);
        }

        let mut matches = 0;
        for num in player_nums {
            if winners.contains(num) {
                matches += 1;
            }
        }
        match matches {
            0 => continue,
            1 => score += 1,
            2 => score += 2,
            _ => score += calc_score(matches),
        }
    }

    Some(score)
}

fn calc_score(matches: i32) -> u32 {
    let mut score = 2;
    for _ in 3..=matches {
        score *= 2;
    }
    score
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards_count: HashMap<u32, u32> = HashMap::new();
    let mut card_matches: HashMap<u32, Vec<u32>> = HashMap::new();
    let card_num_regex = Regex::new(r"d\s*(\d+):").unwrap();
    for card_line in input.split("\n") {
        let card_number: u32 = card_num_regex
            .captures(card_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let matches = get_matches(card_line, card_number);
        cards_count.insert(card_number, 1);
        card_matches.insert(card_number, matches);
    }

    for c in card_matches.clone().into_keys().into_iter().sorted() {
        eval_card(&c, &card_matches, &mut cards_count);
    }

    Some(cards_count.into_values().into_iter().sum())
}

fn eval_card(
    card: &u32,
    card_matches: &HashMap<u32, Vec<u32>>,
    mut cards_count: &mut HashMap<u32, u32>,
) {
    let matches = card_matches.get(&card).unwrap();
    for c in matches {
        cards_count.entry(*c).and_modify(|count| *count += 1);
        eval_card(c, card_matches, &mut cards_count);
    }
}

fn get_matches(card_line: &str, card_num: u32) -> Vec<u32> {
    let winning_regex = Regex::new(r":(.*?)\|").unwrap();
    let player_regex = Regex::new(r"\|(.*?)$").unwrap();
    let mut winners: HashSet<&str> = HashSet::new();
    let winner_nums: Vec<&str> = winning_regex
        .captures(card_line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(" ")
        .filter(|c| !c.is_empty())
        .collect();
    let player_nums: Vec<&str> = player_regex
        .captures(card_line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .trim_end()
        .split(" ")
        .filter(|c| !c.is_empty())
        .collect();

    for n in winner_nums {
        winners.insert(n);
    }

    let mut matches: u32 = 0;
    for num in player_nums {
        if winners.contains(num) {
            matches += 1;
        }
    }
    if matches == 0 {
        return vec![];
    }
    ((card_num + 1)..=(card_num + matches)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
