use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let game_regex = Regex::new(r"Game*(.+):").unwrap();
    let red_regex = Regex::new(r"(..) red").unwrap();
    let blue_regex = Regex::new(r"(..) blue").unwrap();
    let green_regex = Regex::new(r"(..) green").unwrap();
    let games = input.split("\n");
    let mut total = 0;
    for game in games {
        let captures = game_regex.captures(game).unwrap();
        let game_id: u32 = captures.get(1).unwrap().as_str().trim().parse().unwrap();
        let reds: Vec<_> = red_regex
            .captures_iter(game)
            .map(|m| m.get(1).unwrap().as_str().trim())
            .collect();
        let blues: Vec<_> = blue_regex
            .captures_iter(game)
            .map(|m| m.get(1).unwrap().as_str().trim())
            .collect();
        let greens: Vec<_> = green_regex
            .captures_iter(game)
            .map(|m| m.get(1).unwrap().as_str().trim())
            .collect();
        let red_total: Vec<_> = reds.iter().map(|n| n.to_owned().parse().unwrap()).collect();
        let blue_total: Vec<_> = blues
            .iter()
            .map(|n| n.to_owned().parse().unwrap())
            .collect();
        let green_total: Vec<_> = greens
            .iter()
            .map(|n| n.to_owned().parse().unwrap())
            .collect();

        let red_max: u8 = *red_total.iter().max().unwrap();
        let blue_max: u8 = *blue_total.iter().max().unwrap();
        let green_max: u8 = *green_total.iter().max().unwrap();

        if red_max > 12 || blue_max > 14 || green_max > 13 {
            continue;
        }
        total += game_id;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let red_regex = Regex::new(r"(..) red").unwrap();
    let blue_regex = Regex::new(r"(..) blue").unwrap();
    let green_regex = Regex::new(r"(..) green").unwrap();
    let games = input.split("\n");
    let mut total = 0;
    for game in games {
        let reds: Vec<_> = red_regex
            .captures_iter(game)
            .map(|m| m.get(1).unwrap().as_str().trim())
            .collect();
        let blues: Vec<_> = blue_regex
            .captures_iter(game)
            .map(|m| m.get(1).unwrap().as_str().trim())
            .collect();
        let greens: Vec<_> = green_regex
            .captures_iter(game)
            .map(|m| m.get(1).unwrap().as_str().trim())
            .collect();
        let red_total: Vec<_> = reds.iter().map(|n| n.to_owned().parse().unwrap()).collect();
        let blue_total: Vec<_> = blues
            .iter()
            .map(|n| n.to_owned().parse().unwrap())
            .collect();
        let green_total: Vec<_> = greens
            .iter()
            .map(|n| n.to_owned().parse().unwrap())
            .collect();

        let red_max: u32 = *red_total.iter().max().unwrap();
        let blue_max: u32 = *blue_total.iter().max().unwrap();
        let green_max: u32 = *green_total.iter().max().unwrap();

        let cube_power: u32 = red_max * blue_max * green_max;

        total += cube_power;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
