use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.to_owned().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut total = 0;
    for h in histories {
        let extrapolated = get_extrapolated(h);
        let mut last_diff = 0;
        for l in (0..extrapolated.len() - 1).rev() {
            last_diff = extrapolated[l].last().unwrap() + last_diff;
        }
        total += last_diff
    }

    Some(total)
}

fn get_extrapolated(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut extrap_vec = vec![];
    let mut found_zero = false;
    let mut current_lvl = 0;
    extrap_vec.push(history);

    while !found_zero {
        let mut level_vec: Vec<i32> = vec![];
        for i in 0..extrap_vec[current_lvl].len() - 1 {
            level_vec.push(extrap_vec[current_lvl][i + 1] - extrap_vec[current_lvl][i]);
        }
        extrap_vec.push(level_vec.clone());

        if level_vec.iter().sum::<i32>() == 0
            && level_vec.iter().unique().collect::<Vec<_>>().len() == 1
        {
            found_zero = true;
        }

        current_lvl += 1;
    }

    extrap_vec
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.to_owned().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut total = 0;
    for h in histories {
        let extrapolated = get_extrapolated(h);
        let mut last_diff = 0;
        for l in (0..extrapolated.len() - 1).rev() {
            last_diff = extrapolated[l][0] - last_diff;
        }
        total += last_diff
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
