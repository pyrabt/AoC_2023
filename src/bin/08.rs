use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut get_instruction = true;
    let mut instructions = "".to_owned();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() && get_instruction {
            get_instruction = false;
            continue;
        }
        if get_instruction {
            instructions.push_str(line);
        } else {
            let node_name = line[0..=2].to_owned();
            let node_left = line[7..=9].to_owned();
            let node_right = line[12..=14].to_owned();
            nodes.insert(node_name, (node_left, node_right));
        }
    }

    let instruction_chars: Vec<char> = instructions.chars().collect();
    let mut in_index = 0;
    let mut steps = 0;
    let mut current_node: String = "AAA".to_owned();
    while current_node != "ZZZ".to_owned() {
        let instruction = instruction_chars[in_index];
        if instruction == 'L' {
            current_node = nodes.get(&current_node).unwrap().0.to_owned();
        } else {
            current_node = nodes.get(&current_node).unwrap().1.to_owned();
        }
        steps += 1;
        in_index = (in_index + 1) % instruction_chars.len();
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut get_instruction = true;
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() && get_instruction {
            get_instruction = false;
            continue;
        }
        if !get_instruction {
            let node_name = line[0..=2].to_owned();
            let node_left = line[7..=9].to_owned();
            let node_right = line[12..=14].to_owned();
            nodes.insert(node_name, (node_left, node_right));
        }
    }

    let a_nodes: Vec<_> = nodes.keys().filter(|k| k.to_owned().ends_with("A")).collect();
    let num_a = a_nodes.len();
    let mut total_steps: Vec<_> = vec![];
    let mut count = 0;
    for node in a_nodes {
        count += 1;
        println!("Node: {node} ({}/{})", count, num_a);
        total_steps.push(get_steps(node.to_owned(), &nodes, 0, vec![]).unwrap());
    }

    println!("{:?}", total_steps);
    Some(lcm(total_steps))
}

fn lcm(numbers: Vec<u32>) -> u32 {
    let mut temp = numbers.clone();

    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).map(|(index, _)| index) {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            }
            None => panic!("Not possible")
        }
    }
}

fn get_steps(current_node: String, nodes: &HashMap<String, (String, String)>, steps: u32, mut visited: Vec<String>) -> Option<u32> {
    let node = nodes.get(current_node.as_str()).unwrap();
    if node.0.ends_with("Z") || node.1.ends_with("Z") {
        return Some(steps + 1);
    }

    if visited.contains(&current_node) { return None; }

    visited.push(current_node.clone());
    let left = match get_steps(nodes.get(&current_node).unwrap().0.to_owned(), &nodes, steps + 1, visited.clone()) {
        None => 0,
        Some(val) => val
    };

    let right = match get_steps(nodes.get(&current_node).unwrap().1.to_owned(), &nodes, steps + 1, visited.clone()) {
        None => 0,
        Some(val) => val
    };

    if left > right {
        return Some(left);
    }

    Some(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
