use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn parse_input(file_path: &str) -> (Vec<i32>, Vec<i32>) { 
    let mut input = String::new();
    let mut f = File::open(file_path).unwrap();
    f.read_to_string(&mut input).unwrap();

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for l in input.lines() {
        let ids: Vec<&str> = l.split_whitespace().collect();
        v1.push(i32::from_str_radix(ids[0], 10).unwrap());
        v2.push(i32::from_str_radix(ids[1], 10).unwrap());
    }
    (v1, v2)
}

fn part1(file_path: &str) -> i32 {
    let (mut group_one_ids, mut group_two_ids) = parse_input(file_path);

    group_one_ids.sort();
    group_two_ids.sort();

    let mut total_distance = 0;

    for i in 0..group_one_ids.len() {
        let difference = (group_one_ids[i] - group_two_ids[i]).abs();
        total_distance += difference;
    }
    total_distance
}

fn part2(file_path: &str) -> i32 {
    let (group_one_ids, group_two_ids) = parse_input(file_path);

    let mut similarity_score = 0;
    let mut appearances: HashMap<i32, i32> = HashMap::new();

    for &n in group_two_ids.iter() {
        if let Some(v) = appearances.get(&n) {
            appearances.insert(n, v + 1);
        } else {
            appearances.insert(n, 1);
        }
    }

    for &m in group_one_ids.iter() {
        if let Some(v) = appearances.get(&m) {
            similarity_score += m * v;
        }
    }
    similarity_score
}

fn main() {
    println!("Total distance: {}", part1("src/day01/input.txt"));
    println!("Similarity score : {}", part2("src/day01/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11, part1("src/day01/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, part2("src/day01/test.txt"));
    }
}
