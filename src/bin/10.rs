use num::integer::div_ceil;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<String>> = vec![];
    for line in input.lines() {
        map.push(line.split("").filter(|s| !s.is_empty()).map(|s| s.to_owned()).collect());
    }

    // get start indices
    let mut s_yx = (0, 0);
    for l in 0..map.len() {
        if map[l].contains(&"S".to_owned()) {
            s_yx = (l, map[l].iter().position(|x| x == &"S".to_owned()).unwrap());
            break;
        }
    }

    let mut current: (usize, usize);
    let mut next: (usize, usize) = (0, 0);
    match start_next_pipe(s_yx, &map) {
        Some(val) => next = val[0],
        None => println!("no path found")
    };

    let mut last = s_yx;
    let mut steps: Vec<String> = vec![];
    current = next;

    let mut dead_end = false;
    while !dead_end {
        match get_next(current, last, &map) {
            Some(val) => next = val,
            None => dead_end = true
        };
        steps.push(map[current.0][current.1].clone());
        last = current;
        current = next;
        if current == s_yx { break; }
    }

    return Some(div_ceil(steps.len(), 2) as u32);
}

fn start_next_pipe(current: (usize, usize), map: &Vec<Vec<String>>) -> Option<Vec<(usize, usize)>> {
    let mut options: Vec<(usize, usize)> = vec![];
    if current.1 > 0 {
        let lt_pipe = map[current.0][current.1 - 1].clone();
        if lt_pipe.eq("-") || lt_pipe.eq("F") || lt_pipe.eq("L") { options.push((current.0, current.1 - 1)); }
    }
    if current.0 > 0 {
        let lt_pipe = map[current.0 - 1][current.1].clone();
        if lt_pipe.eq("|") || lt_pipe.eq("F") || lt_pipe.eq("7") { options.push((current.0 - 1, current.1)); }
    }
    if current.0 < map.len() - 1 {
        let lt_pipe = map[current.0 + 1][current.1].clone();
        if lt_pipe.eq("|") || lt_pipe.eq("L") || lt_pipe.eq("J") { options.push((current.0 + 1, current.1)); }
    }
    if current.1 < map[0].len() - 1 {
        let lt_pipe = map[current.0][current.1 + 1].clone();
        if lt_pipe.eq("-") || lt_pipe.eq("J") || lt_pipe.eq("7") { options.push((current.0, current.1 + 1)); }
    }
    if map.len() > 0 {
        return Some(options);
    }

    None
}

fn get_next(current: (usize, usize), last: (usize, usize), map: &Vec<Vec<String>>) -> Option<(usize, usize)> {
    let pipe_type = map[current.0][current.1].clone();
    if pipe_type.eq("|") {
        //up
        if last.0 != current.0 - 1 && current.0 > 0 { return Some((current.0 - 1, current.1)); }
        //down
        if current.0 < map.len() - 1 { return Some((current.0 + 1, current.1)); }
    } else if pipe_type.eq("-") {
        //left
        if last.1 != current.1 - 1 && current.1 > 0 { return Some((current.0, current.1 - 1)); }
        //right
        if current.1 < map[0].len() - 1 { return Some((current.0, current.1 + 1)); }
    } else if pipe_type.eq("7") {
        //left
        if last.1 != current.1 - 1 && current.1 > 0 { return Some((current.0, current.1 - 1)); }
        //down
        if current.0 < map.len() - 1 { return Some((current.0 + 1, current.1)); }
    } else if pipe_type.eq("L") {
        //up
        if last.0 != current.0 - 1 && current.0 > 0 { return Some((current.0 - 1, current.1)); }
        //right
        if current.1 < map[0].len() - 1 { return Some((current.0, current.1 + 1)); }
    } else if pipe_type.eq("J") {
        //up
        if last.0 != current.0 - 1 && current.0 > 0 { return Some((current.0 - 1, current.1)); }
        //left
        if current.1 > 0 { return Some((current.0, current.1 - 1)); }
    } else if pipe_type.eq("F") {
        //right
        if last.1 != current.1 + 1 && current.1 < map[0].len() - 1 { return Some((current.0, current.1 + 1)); }
        //down
        if current.0 < map.len() - 1 { return Some((current.0 + 1, current.1)); }
    }

    None
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
