advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut data: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        let nums: Vec<_> = line
            .trim()
            .split(" ")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.to_owned())
            .filter(|s| {
                !s.is_empty()
                    && match s.parse::<u32>() {
                        Ok(_) => true,
                        Err(_) => false,
                    } == true
            })
            .map(|s| s.to_owned().parse::<u32>().unwrap())
            .collect();
        data.push(nums);
    }

    let mut total = 1;
    for race_index in 0..data[0].len() {
        let mut count = 0;
        for x in 1..data[0][race_index] {
            if x * (data[0][race_index] - x) > data[1][race_index] {
                count += 1;
            }
        }
        total *= count;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let [ref time, ref distance]: _ = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| {
            l.trim().split(":").collect::<Vec<_>>()[1]
                .replace(" ", "")
                .to_owned()
        })
        .collect::<Vec<String>>()[..]
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!("Pattern doesn't match input.")
    };

    let mut count = 0;
    for x in 1..*time {
        if x * (time - x) > *distance {
            count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
