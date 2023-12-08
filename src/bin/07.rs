use std::cmp::Ordering;
use itertools::Itertools;
use fancy_regex::Regex;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = vec![];
    for line in input.lines() {
        let [ref hand, ref bet] = line.split(" ").map(|s| s.to_owned()).collect::<Vec<_>>()[..]
        else {
            panic!("Line {line} doesn't match pattern")
        };
        hands.push((get_score(hand), (hand.to_owned(), bet.parse::<u32>().unwrap())));
    }

    hands.sort_by(|a, b| hand_ordering(a, b));

    let mut total:u32 = 0;
    for i in 0..hands.len(){
        println!("{} ({})- ({} + 1) * {}", hands[i].1.0, hands[i].0, i as u32, hands[i].1.1);
        total += (i as u32 + 1) * hands[i].1.1;
    }
    Some(total)
}

fn hand_ordering(hand_a: &(i32, (String, u32)), hand_b: &(i32, (String, u32))) -> Ordering{
    let card_ranks: Vec<_> = "123456789TJQKA".chars().collect();
    let a_chars: Vec<_> = hand_a.1.0.chars().collect();
    let b_chars: Vec<_> = hand_b.1.0.chars().collect();
    if hand_a.0 > hand_b.0{
        return Ordering::Greater;
    } else if hand_a.0 < hand_b.0 {
        return Ordering::Less;
    }
    for i in 0..=5{
        let a_rank = card_ranks.iter().position(|&x| x == a_chars[i]).unwrap();
        let b_rank = card_ranks.iter().position(|&x| x == b_chars[i]).unwrap();
        if a_rank > b_rank{
            return Ordering::Greater;
        }
        if a_rank < b_rank {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn get_score(hand: &String) -> i32 {
    if get_ofkind_regex(5).is_match(hand).unwrap() { return 7;}
    if get_ofkind_regex(4).is_match(hand).unwrap() {return 6;}
    if hand.chars().unique().collect::<Vec<_>>().len() == 2 {return 5;}
    if get_ofkind_regex(3).is_match(hand).unwrap() {return 4;}
    if hand.chars().unique().collect::<Vec<_>>().len() == 3 {return 3;}
    if get_ofkind_regex(2).is_match(hand).unwrap() {return 2;}
    1
}

fn get_ofkind_regex(num_same: i32) -> Regex {
    let mut pair_string = format!(
        "A{{{num_same}}}|K{{{num_same}}}|Q{{{num_same}}}|J{{{num_same}}}|T{{{num_same}}}|(\\d).*?(\\1)");
    for _ in 2..num_same{
        pair_string.push_str(r".*?(\1)");
    }
    Regex::new(pair_string.as_str()).unwrap()
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
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
