advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input_sections = get_fields(input);
    let seeds = &input_sections[0];

    let mut closest: u32 = 0;
    for s in seeds {
        let seed_loc= get_location(*s, &input_sections);

        if closest == 0 || closest > seed_loc {
            closest = seed_loc;
        }
    }

    Some(closest)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_sections = get_fields(input);
    let seeds = &input_sections[0];

    let mut closest: u32 = 0;

    for seed_range in seeds.chunks_exact(2){
        for s in seed_range[0]..(seed_range[0] + seed_range[1]) {
            let s_loc = get_location(s, &input_sections);
            if closest == 0 || closest > s_loc {
                closest = s_loc;
            }
        }
    }

    Some(closest)
}

fn get_fields(input: &str) -> Vec<Vec<u32>> {
    let mut input_sections: Vec<_> = vec![];
    let mut temp_vec = "".to_owned();
    for l in input.lines() {
        let line = l.to_owned();
        if line.is_empty() || line == "end" {
            let data: Vec<_> = temp_vec
                .trim_start()
                .split(" ")
                .map(|s| s.to_owned())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            input_sections.push(data);
            temp_vec = "".to_owned();
            continue;
        }
        if line.contains(":") {
            let sep_index = line.find(":").unwrap();
            temp_vec.push_str(&line[sep_index + 1..line.len()])
        } else {
            temp_vec = format!("{} {}", temp_vec, line);
        }
    }
    input_sections
}

fn get_location(seed: u32, map_sections: &Vec<Vec<u32>>) -> u32{
    let mut current_dest: u32 = seed;
    for m in 1..=7 {
        let map_data = &map_sections[m];
        for set in map_data.chunks_exact(3) {
            let [dest, source, range] = set else {
                panic!("Map set pattern couldn't be matched!");
            };
            if current_dest >= *source && current_dest <= (source + range-1) {
                current_dest = dest + (current_dest-source);
                break;
            }
        }
    }
    current_dest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}