use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn parse_input(file_path: &str) -> (Vec<(i32,i32)>, Vec<Vec<i32>>) {
    let mut input = String::new();
    let mut f = File::open(file_path).unwrap();
    f.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let mut rules: Vec<(i32, i32)> = Vec::new();
    loop {
        match lines.next() {
            Some("") => { break; },
            Some(s) => {
                let pair_vec: Vec<i32> = s.split("|")
                    .map(|e| i32::from_str_radix(e, 10).unwrap())
                    .collect();
                assert_eq!(pair_vec.len(), 2);
                rules.push((pair_vec[0], pair_vec[1])); 
            },
            None => panic!("Invalid input"),
        }
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();
    loop {
        if let Some(s) = lines.next() {
            let update: Vec<i32> = s.split(",")
                    .map(|e| i32::from_str_radix(e, 10).unwrap())
                    .collect();
            updates.push(update);
        } else {
            break; // EOF
        }
    }
    (rules, updates)
}

fn part1(file_path: &str) -> i32 {
    let (rules, updates) = parse_input(file_path);
    let rule_map = generate_rule_map(&rules);
    let mut total_sum: i32 = 0;

    for update in updates.iter() {
        if follows_rules(update, &rule_map) {
            let idx: usize = update.len() / 2;
            total_sum += update[idx] 
        }
    }
    total_sum
}

fn generate_rule_map(rules: &Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for &rule in rules.iter() {
        if let Some(v) = rule_map.get_mut(&rule.0) {
            v.push(rule.1);
        } else {
            rule_map.insert(rule.0, vec![rule.1]);
        }
    }
    rule_map
}

fn follows_rules(update: &Vec<i32>, rule_map: &HashMap<i32,Vec<i32>>) -> bool {
    for i in 1..update.len() {
        let rules = match rule_map.get(&update[i]) {
            Some(list) => list,
            None => continue,
        };

        for j in 0..i {
            if rules.contains(&update[j]) {
                return false;
            }
        }
    }
    return true;
}

fn part2(file_path: &str) -> i32 {
    let (rules, mut updates) = parse_input(file_path);
    let rule_map = generate_rule_map(&rules);
    let mut total_sum: i32 = 0;

    for update in updates.iter_mut() {
        if !follows_rules(update, &rule_map) {
            // Order the updates
            order_update(update, &rule_map);

            let idx: usize = update.len() / 2;
            total_sum += update[idx] 
        }
    }
    total_sum
}

fn order_update(update: &mut Vec<i32>, rule_map: &HashMap<i32, Vec<i32>>) {
    for i in 1..update.len() {
        let rules = match rule_map.get(&update[i]) {
            Some(list) => list,
            None => continue,
        };

        let mut new_idx = i;
        for j in (0..i).rev() {
            if rules.contains(&update[j]) {
                new_idx = j;
            }
        }
        if new_idx < i {
            let temp = update.remove(i);
            update.insert(new_idx, temp);
        }
    }
    assert!(follows_rules(update, rule_map));
}

fn main() {
    println!("Sum of valid updates: {}", part1("src/day05/input.txt"));
    println!("Sum of invalid updates (fixed): {}", part2("src/day05/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(143, part1("src/day05/test.txt")) 
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, part2("src/day05/test.txt")) 
    }
}
