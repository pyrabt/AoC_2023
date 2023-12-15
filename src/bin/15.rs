use std::collections::HashMap;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.lines(){
        let sequence: Vec<_> = line.split(",").collect();
        for s in sequence{
            sum += compute_hash(s.to_owned());
        }
    }
    Some(sum)
}

fn compute_hash(characters: String) -> usize {
    let ascii = characters.bytes();
    let mut current: usize = 0;
    for c in ascii{
        current += c as usize;
        current *= 17;
        current = current % 256;
    }

    current
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    for line in input.lines(){
        let sequence: Vec<_> = line.split(",").collect();
        for s in sequence{
            if s.contains("-"){
                let input: &str = s.split("-").collect::<Vec<_>>()[0];
                let box_num = compute_hash(input.to_owned());
                if boxes.contains_key(&box_num){
                    let bx = boxes.get_mut(&box_num).unwrap();
                    bx.retain(|x| x.0 != input);
                }
            } else {
                let inputs: Vec<_> = s.split("=").collect();
                let focal_len = inputs[1].parse::<usize>().unwrap();
                let box_num = compute_hash(inputs[0].to_owned());
                if boxes.contains_key(&box_num){
                    let bx = boxes.get_mut(&box_num).unwrap();
                    if bx.iter().filter(|x| x.0 == inputs[0]).collect::<Vec<_>>().len() > 0{
                        let idx = bx.iter().position(|x| x.0 == inputs[0]).unwrap();
                        bx[idx].1 = focal_len;
                    }else { bx.push((inputs[0], focal_len)); }
                }else {
                    let mut lenses: Vec<(&str, usize)> = vec![];
                    lenses.push((inputs[0], focal_len));
                    boxes.insert(box_num, lenses); }
            }
        }
    }

    let mut sum = 0;
    for b in boxes{
        let (box_num, lenses) = b;
        for i in 0..lenses.len() {
            sum += (box_num + 1) * (i+1) * lenses[i].1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
