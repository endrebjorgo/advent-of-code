use std::io::Read;
use std::fs::File;

fn parse_input(file_path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(file_path).unwrap();
    f.read_to_string(&mut input).unwrap();
    
    input
}

fn part1(file_path: &str) -> i32 {
    let input = parse_input(file_path);
    let candidates = string_search(&input, "mul(");
    sum_of_products(&input, &candidates)

}

fn sum_of_products(input: &str, candidates: &Vec<usize>) -> i32 {
    let input_bytes = input.as_bytes();
    let mut total: i32 = 0;

    for &i in candidates.iter() {
        let mut bracket_content = String::new();
        let mut closed = false;
        let mut counter = 4; // Start after open bracket

        while i + counter < input.len() {
            let curr_char = input_bytes[i+counter] as char;
            match curr_char {
                '0'..='9' | ',' => {
                    bracket_content.push(curr_char);
                },
                ')' => {
                    closed = true;
                    break;
                },
                _ => { break; }
            }
            counter += 1;
        }
        if !closed { continue; }

        total += bracket_content
                    .split(',')
                    .map(|e| i32::from_str_radix(e, 10).unwrap_or(0))
                    .product::<i32>();
    }
    total
}

fn string_search(s: &str, p: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    if s.len() < p.len() { return indices; }

    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    let length = s_bytes.len() - p_bytes.len() + 1;

    for i in 0..length {
        let mut found_match = true;
        for j in 0..p_bytes.len() {
            if s_bytes[i+j] != p_bytes[j] {
                found_match = false;
                break;
            }
        }
        if found_match {
            indices.push(i);
        }
    }
    indices
}

fn part2(file_path: &str) -> i32 {
    let input = parse_input(file_path);

    let dos = string_search(&input, "do()");
    let donts = string_search(&input, "don't()");
    let muls = string_search(&input, "mul(");
    let candidates = filter_muls(&muls, &dos, &donts);

    sum_of_products(&input, &candidates)
}

fn filter_muls(muls: &Vec<usize>, dos: &Vec<usize>, donts: &Vec<usize>) -> Vec<usize> {
    let mut filtered = Vec::new();

    let mut dos_i = 0;
    let mut donts_i = 0;

    for &m in muls.iter() {
        if m == 0 { filtered.push(m); }

        while dos[dos_i] < m {
            if dos_i+1 == dos.len() || dos[dos_i+1] >= m { 
                break;
            }
            dos_i += 1;
        }

        while donts[donts_i] < m {
            if donts_i+1 == donts.len() || donts[donts_i+1] >= m { 
                break;
            }
            donts_i += 1;
        }

        if donts[donts_i] > m {
            filtered.push(m); 
        } else if dos[dos_i] < m && dos[dos_i] > donts[donts_i] {
            filtered.push(m); 
        }
    }
    filtered
}

fn main() {
    println!("Sum of multiplications: {}", part1("src/day03/input.txt"));
    println!("Sum of enabled multiplications: {}", part2("src/day03/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(161, part1("src/day03/test1.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2("src/day03/test2.txt"));
    }
}
