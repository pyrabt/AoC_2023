use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut schematic: Vec<Vec<&str>> = vec![];
    for line in input.split("\n") {
        schematic.push(line.trim().split_terminator("").skip(1).collect());
    }
    let num_regex = Regex::new("[0-9]").unwrap();

    let mut total = 0;

    for y in 0..schematic.len() {
        let mut x = 0;
        while x < schematic[0].len() {
            if num_regex.is_match(schematic[y][x]) {
                let part_number = get_number(x, y, &schematic);
                if is_valid(x, y, part_number.len(), &schematic) {
                    total += part_number.parse::<u32>().unwrap();
                }
                x = x + (part_number.len());
            } else {
                x = x + 1;
            }
        }
    }

    Some(total)
}

fn get_number(x: usize, y: usize, schematic: &Vec<Vec<&str>>) -> String {
    let num_regex = Regex::new("[0-9]").unwrap();
    let mut number = "".to_owned();
    let mut current = x;
    while current <= schematic[0].len() - 1 {
        if !num_regex.is_match(schematic[y][current]) {
            break;
        }
        number.push_str(schematic[y][current]);
        current += 1;
    }

    number
}

fn is_valid(x: usize, y: usize, num_len: usize, schematic: &Vec<Vec<&str>>) -> bool {
    let max_x = schematic[0].len() - 1;
    let max_y = schematic.len() - 1;
    let symbol_regex = Regex::new(r"[^\d\w\s.]").unwrap();

    for current in x..(x + num_len) {
        //check left
        if current > 0 && symbol_regex.is_match(schematic[y][current - 1]) {
            return true;
        }
        //check up
        else if y > 0 && symbol_regex.is_match(schematic[y - 1][current]) {
            return true;
        }
        //check right
        else if current < max_x && symbol_regex.is_match(schematic[y][current + 1]) {
            return true;
        }
        //check down
        else if y < max_y && symbol_regex.is_match(schematic[y + 1][current]) {
            return true;
        }
        //check up left
        else if y > 0 && current > 0 && symbol_regex.is_match(schematic[y - 1][current - 1]) {
            return true;
        }
        //check up right
        else if y > 0 && current < max_x && symbol_regex.is_match(schematic[y - 1][current + 1]) {
            return true;
        }
        //check down left
        else if y < max_y && current > 0 && symbol_regex.is_match(schematic[y + 1][current - 1]) {
            return true;
        }
        //check down right
        else if y < max_y
            && current < max_x
            && symbol_regex.is_match(schematic[y + 1][current + 1])
        {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut schematic: Vec<Vec<&str>> = vec![];
    for line in input.split("\n") {
        schematic.push(line.trim().split_terminator("").skip(1).collect());
    }

    let mut total = 0;

    for y in 0..schematic.len() {
        for x in 0..schematic[0].len() {
            if schematic[y][x] == "*" {
                let gears = get_unique_gears(x, y, &schematic);

                if gears.len() == 2 {
                    total += gears.iter().product::<u32>();
                }
            }
        }
    }

    Some(total)
}

fn get_gear(gear: &(usize, usize), schematic: &Vec<Vec<&str>>) -> u32 {
    let num_regex = Regex::new("[0-9]").unwrap();
    let mut start_index: usize = gear.1;
    while start_index != 0 && num_regex.is_match(schematic[gear.0][start_index - 1]) {
        start_index = start_index - 1;
    }
    let mut end_index = start_index;
    while end_index != schematic[0].len() && num_regex.is_match(schematic[gear.0][end_index]) {
        end_index = end_index + 1;
    }
    let gear_num = &schematic[gear.0][start_index..end_index].join("");
    gear_num.to_owned().parse::<u32>().unwrap()
}

fn get_unique_gears(x: usize, y: usize, schematic: &Vec<Vec<&str>>) -> Vec<u32> {
    let max_x = schematic[0].len() - 1;
    let max_y = schematic.len() - 1;
    let num_regex = Regex::new("[0-9]").unwrap();
    let mut gears = vec![];

    //check left
    if x > 0 && num_regex.is_match(schematic[y][x - 1]) {
        gears.push((y, x - 1));
    }
    //check up
    if y > 0 && num_regex.is_match(schematic[y - 1][x]) {
        gears.push((y - 1, x));
    }
    //check right
    if x < max_x && num_regex.is_match(schematic[y][x + 1]) {
        gears.push((y, x + 1));
    }
    //check down
    if y < max_y && num_regex.is_match(schematic[y + 1][x]) {
        gears.push((y + 1, x));
    }
    //check up left
    if y > 0 && x > 0 && num_regex.is_match(schematic[y - 1][x - 1]) {
        gears.push((y - 1, x - 1));
    }
    //check up right
    if y > 0 && x < max_x && num_regex.is_match(schematic[y - 1][x + 1]) {
        gears.push((y - 1, x + 1));
    }
    //check down left
    if y < max_y && x > 0 && num_regex.is_match(schematic[y + 1][x - 1]) {
        gears.push((y + 1, x - 1));
    }
    //check down right
    if y < max_y && x < max_x && num_regex.is_match(schematic[y + 1][x + 1]) {
        gears.push((y + 1, x + 1));
    }

    let found_gears: Vec<_> = gears.iter().map(|g| get_gear(&g, &schematic)).collect();
    let unique_gears: Vec<_> = found_gears.into_iter().unique().collect();

    unique_gears
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
