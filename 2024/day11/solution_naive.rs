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

    fn blink_n_times(&mut self, iterations: usize) {
        let mut memo: HashMap<u64, Vec<u64>> = HashMap::new();

        for x in 0..iterations {
            //self.blink_v1();
            self.blink_v2(&mut memo);
        }
    }

    fn blink_v2(&mut self, memo: &mut HashMap<u64, Vec<u64>>) {
        // Better, but still slow...
        let mut idx = 0;
        let mut original_length = self.0.len();

        memo.insert(0, vec![1]);

        while idx < original_length {
            let curr_stone = self.0[idx];

            if let Some(e) = memo.get(&curr_stone) {
                self.0[idx] = e[0];
                if e.len() == 2 { self.0.push(e[1]); }
            } else if number_of_digits(curr_stone) % 2 == 0 {
                let (new1, new2) = split_even_digits(curr_stone);
                self.0[idx] = new1;
                self.0.push(new2);
                memo.insert(curr_stone, vec![new1, new2]);
            } else {
                self.0[idx] = curr_stone * 2024;
                memo.insert(curr_stone, vec![curr_stone * 2024]);
            }
            idx += 1;
        }
    }

    fn blink_v1(&mut self) {
        // Will take about 70 million years for 75 iterations...
        let mut idx = 0;
        while idx < self.0.len() {
            let curr_stone = self.0[idx];
            if curr_stone == 0 {
                self.0[idx] = 1;
                idx += 1;
            } else if number_of_digits(curr_stone) % 2 == 0{
                let (new1, new2) = split_even_digits(curr_stone);
                self.0[idx] = new1;
                self.0.insert(idx + 1, new2);
                idx += 2;
            } else {
                self.0[idx] = curr_stone * 2024;
                idx += 1;
            }
        }
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


fn split_even_digits(n: u64) -> (u64, u64) {
    let num_digits = number_of_digits(n);
    assert!(num_digits % 2 == 0);

    let power = u64::pow(10, num_digits / 2);
    (n / power, n % power)
}

fn part1(file_path: &str) -> usize {
    let mut stones = Stones::from_file(file_path);
    stones.blink_n_times(25);
    stones.0.len()
}

fn part2(file_path: &str) -> usize {
    let mut stones = Stones::from_file(file_path);
    stones.blink_n_times(75);
    stones.0.len()
}

fn test(file_path: &str, iterations: usize) -> usize {
    let mut stones = Stones::from_file(file_path);
    stones.blink_n_times(iterations);
    stones.0.len() 
}

fn main() {
    let mut start = Instant::now();

    for i in 1..=10 {
        start = Instant::now();
        let iterations = i*5;
        println!(
            "Total stones after {} blinks: {} (Duration: {:.2?})",
            iterations,
            test("src/day11/input.txt", iterations),
            start.elapsed()
        );
    }

    /*
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
    */
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
        
    }
}
