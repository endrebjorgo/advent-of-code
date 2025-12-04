use std::fs::File;
use std::io::Read;

fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut input = String::new();
    let mut f = File::open(file_path).unwrap();
    f.read_to_string(&mut input).unwrap();

    let mut output: Vec<Vec<i32>> = Vec::new();

    for l in input.lines() {
        let levels_str: Vec<&str> = l.split_whitespace().collect();
        let levels: Vec<i32> = levels_str.into_iter()
            .map(|e| i32::from_str_radix(e, 10).unwrap())
            .collect();
        output.push(levels);
    }
    output
}

fn part1(file_path: &str) -> i32 {
    let levels = parse_input(file_path);
    let total_safe = levels.iter().map(|e| is_safe(e) as i32).sum();
    total_safe
}

fn is_allowed(distance: i32) -> bool {
    distance <= 3 && distance > 0
}

fn is_safe(levels: &Vec<i32>) -> bool {
    // To check if slowly decreasing, check if the reverse is increasing.
    let levels_rev: Vec<i32> = levels.clone().into_iter().rev().collect();
    is_slowly_increasing(&levels) || is_slowly_increasing(&levels_rev)
}

fn is_slowly_increasing(levels: &Vec<i32>) -> bool {
    for i in 0..(levels.len() - 1) {
        if !is_allowed(levels[i+1] - levels[i]) {
            return false;
        }
    }
    return true;
}

fn part2(file_path: &str) -> i32 {
    let levels = parse_input(file_path);
    let total_safe = levels.iter().map(|e| is_safe_pd(e) as i32).sum();
    total_safe
}

fn is_safe_pd(levels: &Vec<i32>) -> bool {
    let levels_rev: Vec<i32> = levels.clone().into_iter().rev().collect();
    is_slowly_increasing_pd(&levels) || is_slowly_increasing_pd(&levels_rev)
}

fn get_element_distances(elements: &Vec<i32>) -> Vec<i32> {
    let indices: Vec<usize> = (0..(elements.len() - 1)).collect();
    let distances: Vec<i32> = indices.into_iter()
        .map(|i| elements[i+1] - elements[i])
        .collect();
    distances
}

fn is_slowly_increasing_pd(levels: &Vec<i32>) -> bool {
    let distances = get_element_distances(&levels);

    let mut bad_distance_indices: Vec<usize> = Vec::new();
    for i in 0..distances.len() {
        if !is_allowed(distances[i]) {
            bad_distance_indices.push(i);
        }
    }

    // Removing a 'bad' distance involves replacing it by the sum of itself and 
    // an adjacent distance, resulting in the distance between either i-1 and 
    // i+1, or i and i+2 (removing i or i+1).
    match bad_distance_indices.len() {
        0 => { return true; },
        1 => {
            // If there is only one bad distance and it is on an edge, it can 
            // always be resolved.
            let bad_i = bad_distance_indices[0];
            if bad_i == 0 || bad_i == distances.len() - 1 {
                return true;
            } else {
                return is_allowed(distances[bad_i-1] + distances[bad_i]) ||
                       is_allowed(distances[bad_i] + distances[bad_i+1]);
            }
        },
        2 => {
            // The bad distances must be adjacent and sum to a good distance.
            let bad_1 = bad_distance_indices[0];
            let bad_2 = bad_distance_indices[1];
            if bad_2 - bad_1 == 1 {
                return is_allowed(distances[bad_1] + distances[bad_2]);
            } else {
                return false;
            }
        },
        _ => { return false; }
    }
}

fn main() {
    println!("Safe reports: {}", part1("src/day02/input.txt"));
    println!("Safe reports w/ Dampener: {}", part2("src/day02/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1("src/day02/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2("src/day02/test.txt"));
    }
}

