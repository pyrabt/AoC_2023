use std::collections::HashSet;
use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    let winning_regex = Regex::new(r"\:(.*?)\|").unwrap();
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

        for n in winner_nums{
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
