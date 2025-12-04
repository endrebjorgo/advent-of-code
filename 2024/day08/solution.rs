use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Default, Debug)]
struct Map {
    height: usize,
    width: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    fn from_file(file_path: &str) -> Self {
        let mut input = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut input).unwrap();

        let mut map = Self::default();

        let mut row_number: usize = 0;
        for line in input.lines() {
            let mut col_number: usize = 0; 
            for c in line.chars() {
                if c != '.' {
                    if let Some(list) = map.antennas.get_mut(&c) {
                        list.push((col_number, row_number));
                    } else {
                        map.antennas.insert(c, vec![(col_number, row_number)]);
                    }
                }
                col_number += 1; 
            }
            row_number += 1;
        }
        map.height = row_number;
        map.width = input.lines().next().unwrap().len();
        map
    }
}

fn part1(file_path: &str) -> i32 {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let map = Map::from_file(file_path);

    for (_, locations) in map.antennas.iter() {
        let valid_nodes = get_antinodes(locations, &map); 
        for &node in valid_nodes.iter() {
            antinodes.insert(node);
        }
    }
    antinodes.len().try_into().unwrap()
}

fn get_antinodes(locations: &Vec<(usize, usize)>, map: &Map) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for i in 1..locations.len() {
        for j in 0..i {
            let p1 = locations[i];
            let p2 = locations[j];

            let cand1_x = 2*p1.0 as i32 - p2.0 as i32;
            let cand1_y = 2*p1.1 as i32 - p2.1 as i32;
            let cand2_x = 2*p2.0 as i32 - p1.0 as i32;
            let cand2_y = 2*p2.1 as i32 - p1.1 as i32;

            if cand1_x >= 0 && cand1_x < map.width as i32 && 
                cand1_y >= 0 && cand1_y < map.height as i32 {
                    antinodes.insert((cand1_x as usize, cand1_y as usize));
            }
            
            if cand2_x >= 0 && cand2_x < map.width as i32 && 
                cand2_y >= 0 && cand2_y < map.height as i32 {
                    antinodes.insert((cand2_x as usize, cand2_y as usize));
            }
        }
    }
    antinodes
}

fn part2(file_path: &str) -> i32 {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let map = Map::from_file(file_path);

    for (_, locations) in map.antennas.iter() {
        let valid_nodes = get_inline_antinodes(locations, &map); 
        for &node in valid_nodes.iter() {
            antinodes.insert(node);
        }
    }
    antinodes.len().try_into().unwrap()
}

fn get_inline_antinodes(locations: &Vec<(usize, usize)>, map: &Map) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for i in 1..locations.len() {
        for j in 0..i {
            let p1 = locations[i];
            let p2 = locations[j];

            let dx = p2.0 as i32 - p1.0 as i32;
            let dy = p2.1 as i32 - p1.1 as i32;

            let mut count: i32 = 0;
            loop {
                let cand1_x = p1.0 as i32 - count*dx;
                let cand1_y = p1.1 as i32 - count*dy;

                if cand1_x < 0 || cand1_x >= map.width as i32 || 
                    cand1_y < 0 || cand1_y >= map.height as i32 {
                        break;
                }
                antinodes.insert((cand1_x as usize, cand1_y as usize));
                count += 1;
            }

            count = 0;

            loop {
                let cand2_x = p2.0 as i32 + count*dx as i32;
                let cand2_y = p2.1 as i32 + count*dy as i32;

                if cand2_x < 0 || cand2_x >= map.width as i32 || 
                    cand2_y < 0 || cand2_y >= map.height as i32 {
                        break;
                }
                antinodes.insert((cand2_x as usize, cand2_y as usize));
                count += 1;
            }
        }
    }
    antinodes
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Total antinode locations: {} (Duration: {:.2?})",
        part1("src/day08/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Total antinode locations (new method): {} (Duration: {:.2?})",
        part2("src/day08/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(14, part1("src/day08/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(34, part2("src/day08/test.txt"));
    }
}
