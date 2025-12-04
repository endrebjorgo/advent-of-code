use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Default)]
struct Maze {
    start_pos: (usize, usize),
    start_dir: (i32, i32),
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
}

impl Maze {
    fn from_file(file_path: &str) -> Self {
        let mut maze = Maze::default();

        let mut buf = Vec::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_end(&mut buf).unwrap();

        let mut row = 0;
        let mut col = 0;

        for &c in buf.iter() {
            if c == b'\n' {
                row += 1;
                col = 0;
                continue;
            }

            match c {
                b'#' => { maze.walls.push((col, row)); },
                b'S' => { maze.start_pos = (col, row); },
                b'E' => { maze.goal = (col, row); },
                b'.' => {},
                _ => unreachable!(),
            }
            col += 1;
        }
        maze.start_dir = (1, 0);
        maze
    }

    fn heur(&self, pos: (usize, usize)) -> usize {
        let dx = (self.goal.0 as i32 - pos.0 as i32).abs() as usize;
        let dy = (self.goal.1 as i32 - pos.1 as i32).abs() as usize;
        return dx + dy + 2000; // Manhattan distance plus 2 turns
    }

    fn get_cheapest_path(&self) -> Option<usize> {
        let mut pq: PQueue<((usize, usize), (i32, i32))> = PQueue::new();
        pq.insert((self.start_pos, self.start_dir), self.heur(self.start_pos));
        
        let mut visited: HashSet<((usize, usize), (i32, i32))> = HashSet::new();

        while let Some(((curr_pos, curr_dir), f)) = pq.pop() {
            let curr_cost = f - self.heur(curr_pos);

            if curr_pos == self.goal { return Some(curr_cost); }

            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    if (x_off == 0) == (y_off == 0) { continue; }

                    // Should not panic as pos is never on edge
                    let new_x = (curr_pos.0 as i32 + x_off) as usize;
                    let new_y = (curr_pos.1 as i32 + y_off) as usize;

                    let new_pos = (new_x, new_y);
                    if self.walls.contains(&new_pos) { continue; }

                    let new_dir = (x_off, y_off);
                    if visited.contains(&(new_pos, new_dir)) { continue; }

                    /*
                    let turns = if curr_dir == new_dir { 0 } 
                    else if curr_dir.0 == -new_dir.0 { 2 }
                    else { 1 };
                    */

                    let turns = match () {
                        _ if new_dir == curr_dir => 0,
                        _ if new_dir == (-curr_dir.0, -curr_dir.1) => 2,
                        _ => 1
                    };

                    let new_f = curr_cost + 1 + turns*1000 + self.heur(new_pos);
                    
                    visited.insert((new_pos, new_dir));
                    pq.insert((new_pos, new_dir), new_f);
                }
            } 
        }
        return None;
    }
}

// (0,1) -> (-1,0) -> (0, -1) -> (1,0)

#[derive(Debug, Default)]
struct PQueue<T> {
    es: Vec<T>,
    fs: Vec<usize>,
}

impl<T> PQueue<T> {
    fn new() -> Self {
        Self {
            es: Vec::new(),
            fs: Vec::new(),
        }
    }

    fn insert(&mut self, e: T, f: usize) {
        for i in 0..self.es.len() {
            if self.fs[i] < f {
                self.es.insert(i, e);
                self.fs.insert(i, f);
                return;
            }
        }
        self.es.push(e);
        self.fs.push(f);
    }

    fn pop(&mut self) -> Option<(T, usize)> {
        if self.es.len() == 0 { return None; }
        let e = self.es.pop().unwrap();
        let f = self.fs.pop().unwrap();
        Some((e, f))
    }
}

fn part1(file_path: &str) -> usize {
    let m = Maze::from_file(file_path);
    m.get_cheapest_path().unwrap()
}

fn part2(file_path: &str) -> usize {
    return 0;
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Lowest possible score: {} (Duration: {:.2?})",
        part1("src/day16/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Lowest possible score: {} (Duration: {:.2?})",
        part2("src/day16/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11048, part1("src/day16/test.txt"));
    }
}
