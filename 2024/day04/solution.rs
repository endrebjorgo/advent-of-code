use std::{fs::File, io::Read};

#[derive(Debug)]
struct Board {
    es: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn from_file(file_path: &str) -> Self {
        let mut input = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut input).unwrap();

        let mut es: Vec<char> = input.chars().collect();
        es.retain(|&e| e != '\n');
        let cols = input.lines().count();
        let rows = es.len() / cols;

        Self { es, rows, cols, }
    }
}

fn part1(file_path: &str) -> i32 {
    let board = Board::from_file(file_path); 
    let mut total_matches: i32 = 0;
    let word = "XMAS";
    let word_chars: Vec<char> = word.chars().collect();

    for i in 0..board.es.len() {
        if board.es[i] != word_chars[0] { continue; }

        let x_pos = i % board.cols;
        let y_pos = i / board.rows;
        
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 { continue; }
                let mut match_found = true; 
                let mut count: i32 = 1;

                while count < word_chars.len() as i32 {
                    let new_x: i32 = x_pos as i32 + x_offset*count;
                    let new_y: i32 = y_pos as i32 + y_offset*count;
                    // Check the bounds!
                    let x_oob = new_x < 0 || new_x >= board.cols as i32;
                    let y_oob = new_y < 0 || new_y >= board.rows as i32;

                    if x_oob || y_oob {
                        match_found = false;
                        break;
                    } 

                    let new_idx = new_x as usize + (new_y as usize)*board.cols;

                    if board.es[new_idx] != word_chars[count as usize] {
                        match_found = false;
                        break;
                    }
                    count += 1;
                }
                if match_found { total_matches += 1; }
            }
        }
    }
    total_matches
}

fn part2(file_path: &str) -> i32 {
    let board = Board::from_file(file_path); 
    let mut total_matches: i32 = 0;

    for i in 0..board.es.len() {
        if board.es[i] != 'A' { continue; }

        let x_pos = i % board.cols;
        let y_pos = i / board.rows;

        let mut corner_chars = Vec::new();
        
        for x_offset in -1..=1 {
            let mut has_broken = false;
            for y_offset in -1..=1 {
                if x_offset == 0 || y_offset == 0 { continue; }

                let new_x: i32 = x_pos as i32 + x_offset;
                let new_y: i32 = y_pos as i32 + y_offset;

                let x_oob = new_x < 0 || new_x >= board.cols as i32;
                let y_oob = new_y < 0 || new_y >= board.rows as i32;

                if x_oob || y_oob {
                    has_broken = true;
                    break;
                } 

                let new_idx = new_x as usize + (new_y as usize)*board.cols;
                let new_char = board.es[new_idx];

                if new_char != 'M' && new_char != 'S' {
                    has_broken = true;
                    break;
                }

                corner_chars.push(new_char);
            }
            if has_broken { break; }
        }
        if corner_chars.len() != 4 { continue; }
        
        // Corner chars offset in order: (-1,-1), (-1, 1), (1, -1), (1, 1)
        // Opposite corners should not be the same.
        if corner_chars[0] != corner_chars[3] && 
            corner_chars[1] != corner_chars[2] {
                total_matches += 1;
        }
    }
    total_matches
}

fn main() {
    println!("Times XMAS appears: {}", part1("src/day04/input.txt"));
    println!("Times X-MAS appears: {}", part2("src/day04/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(18, part1("src/day04/test1.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2("src/day04/test2.txt"));
    }
}
