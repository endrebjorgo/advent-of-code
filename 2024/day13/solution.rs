use std::io::Read;
use std::fs::File;
use std::mem::swap;
use std::time::Instant;

fn parse_input(file_path: &str) -> Vec<ClawMachine> {
    let mut buf = String::new();
    let mut f = File::open(file_path).unwrap();
    f.read_to_string(&mut buf).unwrap();

    let mut result = Vec::new();
    let mut btn_a: (i64, i64) = (0, 0);
    let mut btn_b: (i64, i64) = (0, 0);
    let mut prize: (i64, i64) = (0, 0);

    for line in buf.lines() {
        if line.is_empty() { continue; }

        let (rest, y) = line.split_once(", Y").unwrap();
        let (_, x) = rest.split_once("X").unwrap();
        let tup = (
            i64::from_str_radix(&x[1..], 10).unwrap(),
            i64::from_str_radix(&y[1..], 10).unwrap(),
        );

        match line.chars().nth(7).unwrap() {
            'A' => { btn_a = tup; },
            'B' => { btn_b = tup; },
            'X' => { 
                prize = tup;
                result.push( ClawMachine{ btn_a, btn_b, prize } );
            },
            _ => unreachable!(),
        }
    }
    result
}

#[derive(Debug, Default)]
struct ClawMachine {
    btn_a: (i64, i64),
    btn_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn calculate_press_counts(&self) -> Option<(i64, i64)> {
        let a_num = self.btn_b.1*self.prize.0 - self.btn_b.0*self.prize.1;
        let b_num = self.btn_a.0*self.prize.1 - self.btn_a.1*self.prize.0;
        let den = self.btn_a.0*self.btn_b.1 - self.btn_b.0*self.btn_a.1;

        assert_eq!(self.btn_a.0 * a_num + self.btn_b.0 * b_num, self.prize.0 * den);
        assert_eq!(self.btn_a.1 * a_num + self.btn_b.1 * b_num, self.prize.1 * den);

        // Only whole number presses
        if a_num % den != 0 || b_num % den != 0 {
            return None;
        }

        let a_presses = a_num / den;
        let b_presses = b_num / den;

        // Only non-negative presses
        if a_presses < 0 || b_presses < 0 {
            return None;
        }

        return Some((a_presses, b_presses));
    }
}

fn part1(file_path: &str) -> i64 {
    let mut total_cost = 0;
    let machines = parse_input(file_path);

    for machine in machines.iter() {
        if let Some((a_presses, b_presses)) = machine.calculate_press_counts() {
            total_cost += 3*a_presses + b_presses;
        }
    }
    total_cost
}

fn part2(file_path: &str) -> i64 {
    let mut total_cost = 0;
    let mut machines = parse_input(file_path);

    for machine in machines.iter_mut() {
        machine.prize.0 += 10000000000000; 
        machine.prize.1 += 10000000000000; 
        if let Some((a_presses, b_presses)) = machine.calculate_press_counts() {
            total_cost += 3*a_presses + b_presses;
        }
    }
    total_cost
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Fewest tokens to win: {} (Duration: {:.2?})",
        part1("src/day13/input.txt"),
        start.elapsed()
    );
    
    start = Instant::now();
    println!(
        "Fewest tokens to win after position shift: {} (Duration: {:.2?})",
        part2("src/day13/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(480, part1("src/day13/test.txt"));
    }

    #[test]
    fn test_part2() {
        
    }
}
