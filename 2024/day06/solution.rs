use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};
use std::time::Instant;

#[derive(Default, Debug)]
struct Map {
    height: usize,
    width: usize,
    obstructions: HashSet<(usize, usize)>,
    guard_pos: (usize, usize),
    guard_dir: (i32, i32),
}

impl Map {
    fn from_file(file_path: &str) -> Self {
        let mut input = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut input).unwrap();

        let mut row_count: usize = 0;
        let mut col_count: usize = 0;

        let mut result = Self::default();

        for line in input.lines() {
            for c in line.chars() {
                match c {
                    '^' | '<' | '>' | 'v' => {
                        result.guard_pos = (col_count, row_count);      
                        result.guard_dir = match c {
                            '^' => (0, -1),
                            '>' => (1, 0),
                            'v' => (0, 1),
                            '<' => (-1, 0),
                            _ => unreachable!(),
                        }
                    },
                    '#' => {
                        result.obstructions.insert((col_count, row_count));
                    },
                    _ => {},
                }
                col_count += 1;
            }
            row_count += 1;
            col_count = 0;
        }
        result.height = row_count;
        result.width = input.lines().next().unwrap().len();
        result
    }
}

fn part1(file_path: &str) -> i32 {
    let mut map = Map::from_file(file_path);
    let visited = get_visited_squares(&mut map);
    visited.len().try_into().unwrap()
}

fn get_visited_squares(map: &mut Map) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        visited.insert(map.guard_pos);
        let new_x = map.guard_pos.0 as i32 + map.guard_dir.0;
        let new_y = map.guard_pos.1 as i32 + map.guard_dir.1;
        
        let x_oob = new_x < 0 || new_x >= map.width as i32;
        let y_oob = new_y < 0 || new_y >= map.height as i32;

        if x_oob || y_oob { break; } 

        if map.obstructions.contains(&(new_x as usize, new_y as usize)) {
            let new_x_dir = -map.guard_dir.1;
            let new_y_dir = map.guard_dir.0;
            map.guard_dir = (new_x_dir, new_y_dir);
        } else {
            map.guard_pos = (new_x as usize, new_y as usize);
        }
    }
    visited
}

fn part2(file_path: &str) -> i32 {
    // Still kinda slow...
    let mut map = Map::from_file(file_path);
    let original_pos = map.guard_pos;
    let original_dir = map.guard_dir;
    let candidates = get_visited_squares(&mut map);
    let mut total_looping_obstructions = 0;

    for &square in candidates.iter() {
        if square == original_pos{ continue; }

        map.guard_pos = original_pos;
        map.guard_dir = original_dir;

        map.obstructions.insert(square);

        if contains_loop(&mut map) {
            total_looping_obstructions += 1;
        }
        map.obstructions.remove(&square);
    }
    total_looping_obstructions
}

fn contains_loop(map: &mut Map) -> bool {
    // Recording positions where a turn has been made and the guard's direction.
    let mut snapshots: HashMap<(usize, usize), Vec<(i32, i32)>> = HashMap::new();

    loop {
        let new_x = map.guard_pos.0 as i32 + map.guard_dir.0;
        let new_y = map.guard_pos.1 as i32 + map.guard_dir.1;
        
        let x_oob = new_x < 0 || new_x >= map.width as i32;
        let y_oob = new_y < 0 || new_y >= map.height as i32;

        if x_oob || y_oob { 
            return false;
        } 

        if map.obstructions.contains(&(new_x as usize, new_y as usize)) {
            if let Some(list) = snapshots.get_mut(&map.guard_pos) {
                if list.contains(&map.guard_dir) {
                    return true;
                } else {
                    list.push(map.guard_dir);
                }
            } else {
                snapshots.insert(map.guard_pos, vec![map.guard_dir]);
            }

            let new_x_dir = -map.guard_dir.1;
            let new_y_dir = map.guard_dir.0;
            map.guard_dir = (new_x_dir, new_y_dir);
        } else {
            map.guard_pos = (new_x as usize, new_y as usize);
        }
    }
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Total squares visited: {} (Duration: {:.2?})",
        part1("src/day06/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Possible looping obstructions: {} (Duration: {:.2?})",
        part2("src/day06/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
       assert_eq!(41, part1("src/day06/test.txt"));
    }

    #[test]
    fn test_part2() {
       assert_eq!(6, part2("src/day06/test.txt"));
    }
}
