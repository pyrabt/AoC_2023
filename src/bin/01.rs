advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").filter(|l| !l.is_empty());
    let keys = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut matches: Vec<Vec<&str>> = vec![];
    for line in lines {
        let mut line_matches: Vec<&str> = vec![];
        for i in 0..(line.len()) {
            for key in keys {
                if (i + key.len()) <= line.len() && key == &line[i..(i + key.len())] {
                    line_matches.push(key);
                }
            }
        }
        matches.push(line_matches);
    }

    let mut total = 0;
    for m in matches {
        let first = m.first().unwrap();
        let last = m.last().unwrap();
        let mut left;
        let mut right;
        if first.len() > 1 {
            left = convert_word(first);
        } else {
            left = first;
        }

        if last.len() > 1 {
            right = convert_word(last);
        } else {
            right = last;
        }

        total += (left.to_owned() + right).parse::<u32>().unwrap();
    }
    Some(total)
}

pub fn convert_word(word: &str) -> &'static str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
