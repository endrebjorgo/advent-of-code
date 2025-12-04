use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Default)]
struct Stones(Vec<u64>);

impl Stones {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();
        let stones = buf.split_whitespace()
            .map(|e| u64::from_str_radix(e, 10).unwrap())
            .collect::<Vec<u64>>();
        Self(stones)
    }
}

fn number_of_digits(n: u64) -> u32 {
    // Faster than the log stuff without compiler optimization? 
    if n == 0 { return 1; }

    let mut current = n;
    let mut digits = 0;

    while current > 0 {
        current /= 10;
        digits += 1;
    }
    digits
}

fn split_even_digits(n: u64) -> Vec<u64> {
    let num_digits = number_of_digits(n);
    let power = u64::pow(10, num_digits / 2);
    vec![n / power, n % power]
}

fn total_after_n_blinks(stones: Vec<u64>, blinks: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return stones.len();
    }
    let mut total = 0;

    for &stone in stones.iter() {
        if let Some(n) = memo.get(&(stone, blinks)) {
            total += n;
        } else {
            let stones_after_blink = blink(stone);
            let result = total_after_n_blinks(stones_after_blink, blinks-1, memo);
            memo.insert((stone, blinks), result);
            total += result;
        }
    }
    total
}

fn blink(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    } else if number_of_digits(n) % 2 == 0{
        return split_even_digits(n);
    } else {
        return vec![n * 2024];
    }
}

fn part1(file_path: &str) -> usize {
    let stones = Stones::from_file(file_path);
    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();
    total_after_n_blinks(stones.0, 25, &mut memo)
}

fn part2(file_path: &str) -> usize {
    let stones = Stones::from_file(file_path);
    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();
    total_after_n_blinks(stones.0, 75, &mut memo)
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Total stones after 25 blinks: {} (Duration: {:.2?})",
        part1("src/day11/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Total stones after 75 blinks: {} (Duration: {:.2?})",
        part2("src/day11/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(55312, part1("src/day11/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(65601038650482, part2("src/day11/test.txt")) 
    }
}
