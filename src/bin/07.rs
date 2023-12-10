use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = vec![];
    for line in input.lines() {
        let [ref hand, ref bet] = line.split(" ").map(|s| s.to_owned()).collect::<Vec<_>>()[..]
        else {
            panic!("Line {line} doesn't match pattern")
        };
        hands.push((
            get_score(hand),
            (hand.to_owned(), bet.parse::<u32>().unwrap()),
        ));
    }

    hands.sort_by(|a, b| hand_ordering(false, a, b));

    let mut total: u32 = 0;
    for i in 0..hands.len() {
        println!("{} ({})", hands[i].1 .0, hands[i].0);
        total += (i as u32 + 1) * hands[i].1 .1;
    }
    Some(total)
}

fn hand_ordering(
    joker: bool,
    hand_a: &(i32, (String, u32)),
    hand_b: &(i32, (String, u32)),
) -> Ordering {
    let card_ranks: Vec<_>;
    if joker {
        card_ranks = "1J23456789TQKA".chars().collect();
    } else {
        card_ranks = "123456789TJQKA".chars().collect();
    }
    let a_chars: Vec<_> = hand_a.1 .0.chars().collect();
    let b_chars: Vec<_> = hand_b.1 .0.chars().collect();
    if hand_a.0 > hand_b.0 {
        return Ordering::Greater;
    } else if hand_a.0 < hand_b.0 {
        return Ordering::Less;
    }
    for i in 0..=5 {
        let a_rank = card_ranks.iter().position(|&x| x == a_chars[i]).unwrap();
        let b_rank = card_ranks.iter().position(|&x| x == b_chars[i]).unwrap();
        if a_rank > b_rank {
            return Ordering::Greater;
        }
        if a_rank < b_rank {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn get_score(hand: &String) -> i32 {
    let frequencies: Vec<_> = hand
        .chars()
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        })
        .iter()
        .map(|c| c.1.clone())
        .collect();

    if frequencies.contains(&5) {
        return 7;
    }
    if frequencies.contains(&4) {
        return 6;
    }
    if frequencies.contains(&2) && frequencies.contains(&3) {
        return 5;
    }
    if frequencies.contains(&3) {
        return 4;
    }
    if frequencies
        .iter()
        .filter(|c| c.to_owned() == &2)
        .collect::<Vec<_>>()
        .len()
        == 2
    {
        return 3;
    }
    if frequencies.contains(&2) {
        return 2;
    }
    1
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = vec![];
    for line in input.lines() {
        let [ref hand, ref bet] = line.split(" ").map(|s| s.to_owned()).collect::<Vec<_>>()[..]
        else {
            panic!("Line {line} doesn't match pattern")
        };
        hands.push((
            get_score_jokers(hand),
            (hand.to_owned(), bet.parse::<u32>().unwrap()),
        ));
    }

    hands.sort_by(|a, b| hand_ordering(true, a, b));

    let mut total: u32 = 0;
    for i in 0..hands.len() {
        //println!("{} ({})", hands[i].1.0, hands[i].0);
        total += (i as u32 + 1) * hands[i].1 .1;
    }
    Some(total)
}

fn get_score_jokers(hand: &String) -> i32 {
    let mut freq_map = hand.chars().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    let mut frequencies: Vec<_>;
    if freq_map.contains_key(&'J') && freq_map.len() > 1 {
        let num = freq_map.get(&'J').unwrap().clone();
        freq_map.remove(&'J');
        frequencies = freq_map.values().map(|c| c.clone()).collect::<Vec<_>>();
        let end_index = frequencies.len() - 1;
        frequencies.sort();
        frequencies[end_index] += num;
    } else {
        frequencies = freq_map.values().map(|c| c.clone()).collect::<Vec<_>>();
    }

    if frequencies.contains(&5) {
        return 7;
    }
    if frequencies.contains(&4) {
        return 6;
    }
    if frequencies.contains(&2) && frequencies.contains(&3) {
        return 5;
    }
    if frequencies.contains(&3) {
        return 4;
    }
    if frequencies
        .iter()
        .filter(|c| c.to_owned() == &2)
        .collect::<Vec<_>>()
        .len()
        == 2
    {
        return 3;
    }
    if frequencies.contains(&2) {
        return 2;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
