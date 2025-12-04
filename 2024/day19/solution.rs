use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TowelDesigner {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl TowelDesigner {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        let mut result = Self::default();

        let mut parsing_designs = false;

        for line in buf.lines() {
            if line.is_empty() { 
                parsing_designs = true;
                continue;
            }

            if parsing_designs {
                result.designs.push(line.to_string());
            } else {
                let mut towels: Vec<String> 
                    = line.split(", ").map(|s| s.to_string()).collect();
                result.towels.append(&mut towels);
            }
        }
        result
    }  

    fn count_possible_designs(&self) -> usize {
        let mut count = 0;
        let mut memo: HashMap<&str, bool> = HashMap::new();

        for design in self.designs.iter() {
            if self.is_possible_design(&design, &mut memo) { count += 1; }
        }
        count
    }

    fn is_possible_design<'a>(&self, design: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool {
        if let Some(b) = memo.get(design) {
            return *b;
        }

        if design.len() == 0 { 
            memo.insert("", true);
            return true;
        }

        println!("{}", design);

        for towel in self.towels.iter() {
            if design.len() < towel.len() { continue; }

            let mut found_match = true;

            for i in 0..towel.len() {
                if towel.as_bytes()[i] as char != design.as_bytes()[i] as char {
                    found_match = false;
                    break;
                }
            }
            
            if !found_match { continue; }

            if self.is_possible_design(&design[towel.len()..], memo) {
                memo.insert(design, true);
                return true;
            }
        }
        memo.insert(design, false);
        return false;
    }

    fn count_all_combinations(&self) -> usize {
        let mut count = 0;
        let mut memo: HashMap<&str, usize> = HashMap::new();

        for design in self.designs.iter() {
            count += self.count_combinations(&design, &mut memo);
        }
        count
    }

    fn count_combinations<'a>(&self, design: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(n) = memo.get(design) {
            return *n;
        }

        let mut combos = 0;

        if design.len() == 0 { 
            memo.insert("", 1);
            return 1;
        }

        println!("{}", design);

        for towel in self.towels.iter() {
            if design.len() < towel.len() { continue; }

            let mut found_match = true;

            for i in 0..towel.len() {
                if towel.as_bytes()[i] as char != design.as_bytes()[i] as char {
                    found_match = false;
                    break;
                }
            }
            
            if !found_match { continue; }

            combos += self.count_combinations(&design[towel.len()..], memo);
        }

        memo.insert(design, combos);
        return combos;
    }

}

fn part1(file_path: &str) -> usize {
    let td = TowelDesigner::from_file(file_path);
    td.count_possible_designs()
}

fn part2(file_path: &str) -> usize {
    let td = TowelDesigner::from_file(file_path);
    td.count_all_combinations()
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Possible towel designs: {} (Duration: {:.2?})",
        part1("src/day19/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Possible towel design combos: {} (Duration: {:.2?})",
        part2("src/day19/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6, part1("src/day19/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(16, part2("src/day19/test.txt"));
    }
}
