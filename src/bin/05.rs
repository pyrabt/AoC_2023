use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input_sections = get_fields(input);
    let seeds = &input_sections[0];
    println!("INPUTS RETRIEVED");

    let mut maps: Vec<HashMap<u32, u32>> = vec![];
    for s in 1..=7 {
        maps.push(init_map(&input_sections[s]));
    }

    println!("Maps Created");

    let mut closest: u32 = 0;
    for s in seeds {
        println!("Seed {s}");
        let mut dest: u32 = *s;
        for m in &maps {
            if m.contains_key(&dest) {
                dest = *m.get(&dest).unwrap();
            }
            println!("--> Destination: {dest}");
        }
        if closest == 0 || closest > dest {
            closest = dest;
        }
    }

    Some(closest)
}

fn init_map(data: &Vec<u32>) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();

    for set in 0..data.len() / 3 {
        let start_index = set * 3;
        let [dest, source, range] = data[start_index..start_index + 3] else {
            panic!("Pattern not matched")
        };

        for v in 0..range {
            map.insert(source + v, dest + v);
        }
    }

    map
}

fn get_fields(input: &str) -> Vec<Vec<u32>> {
    let mut input_sections: Vec<_> = vec![];
    let mut temp_vec = "".to_owned();
    for l in input.lines() {
        println!("{l}");
        let line = l.to_owned();
        if line.is_empty() || line == "end" {
            let x: Vec<_> = temp_vec.split(" ").map(|s| s.to_owned()).collect();
            println!("{:?}", x);
            println!("----------------SECTION FINISHED------------");
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
