use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn parse_input(file_path: &str) -> Vec<(u64, Vec<u64>)>{
    let mut input = String::new();
    let mut f = File::open(file_path).unwrap();
    f.read_to_string(&mut input).unwrap();

    let mut output: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in input.lines() {
        let mut outputs = line.split(": ");
        let test_value = u64::from_str_radix(outputs.next().unwrap(), 10)
                            .unwrap();
        let terms: Vec<u64> = outputs
                                .next()
                                .unwrap()
                                .split_whitespace()
                                .map(|e| u64::from_str_radix(e, 10).unwrap())
                                .collect();
        output.push((test_value, terms));
    }
    output
}

fn part1(file_path: &str) -> u64 {
    let input = parse_input(file_path);
    let mut total_valid = 0;

    for (test_value, terms) in input {
        if can_be_true(test_value, terms.as_slice()) {
            total_valid += test_value;
        }
    }
    total_valid
}

fn can_be_true(test_value: u64, terms: &[u64]) -> bool {
    if terms.len() == 1 {
        return test_value == terms[0];
    }
    let remaining_terms = &terms[0..(terms.len()-1)];
    let last_term = *terms.last().unwrap();
    
    if test_value % last_term == 0 {
        if can_be_true(test_value / last_term, &remaining_terms) {
            return true;
        }
    }
    if test_value > last_term {
        if can_be_true(test_value - last_term, &remaining_terms) {
            return true;
        }
    }
    return false;
}

fn part2(file_path: &str) -> u64 {
    let input = parse_input(file_path);
    let mut total_valid = 0;

    for (test_value, terms) in input {
        //println!("{} {:?}", test_value, terms);
        if can_be_true_concat(test_value, terms.as_slice()) {
            total_valid += test_value;
        }
    }
    total_valid
}

fn can_be_true_concat(test_value: u64, terms: &[u64]) -> bool {
    if terms.len() == 1 {
        return test_value == terms[0];
    }
    let last_term = *terms.last().unwrap();
    let remaining = &terms[0..(terms.len()-1)];

    let digits = number_of_digits(last_term);
    let divisor = u64::pow(10, digits);
    
    if test_value % divisor == last_term {
        if can_be_true_concat((test_value - last_term) / divisor, &remaining) {
            return true;
        }
    }
    if test_value % last_term == 0 {
        if can_be_true_concat(test_value / last_term, &remaining) {
            return true;
        }
    }
    if test_value > last_term {
        if can_be_true_concat(test_value - last_term, &remaining) {
            return true;
        }
    }
    return false;
}

fn number_of_digits(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => (n as f64).log10().floor() as u32 + 1,
    }
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Total calibration result: {} (Duration: {:.2?})", 
        part1("src/day07/input.txt"), 
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Total calibration result w/ concat: {} (Duration: {:.2?})",
        part2("src/day07/input.txt"),
        start.elapsed() 
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3749, part1("src/day07/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387, part2("src/day07/test.txt"));
    }
}
