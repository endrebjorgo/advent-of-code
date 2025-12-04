use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(Debug)]
struct MemoryMaze {
    points: Vec<(usize, usize)>,
    h_range: usize,
    v_range: usize,
    fallen: usize
}

impl MemoryMaze{
    fn from_file(file_path: &str, h_range: usize, v_range: usize) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        let mut points: Vec<(usize, usize)> = Vec::new(); 

        for line in buf.lines() {
            let (x_str, y_str) = line.split_once(",").unwrap();
            let point = (
                usize::from_str_radix(x_str, 10).unwrap(),
                usize::from_str_radix(y_str, 10).unwrap()
            );
            points.push(point);
        }
        Self { points, h_range, v_range, fallen: 0 }
    }

    fn heur(&self, point: (usize, usize)) -> usize {
        let dx = (self.v_range as i32 - point.0 as i32).abs() as usize;
        let dy = (self.h_range as i32 - point.1 as i32).abs() as usize;
        return dx + dy;
    }
    
    fn get_shortest_path(&self) -> Option<usize> {
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut pq: Vec<((usize, usize), usize)> 
            = vec![((0, 0), self.heur((0, 0)))];

        while let Some((p, f)) = pq.pop() {
            visited.push(p);
            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    if (x_off == 0) == (y_off == 0) { continue; }

                    let new_x = p.0 as i32 + x_off;
                    let new_y = p.1 as i32 + y_off;

                    let x_oob = new_x < 0 || new_x > self.v_range as i32;
                    let y_oob = new_y < 0 || new_y > self.h_range as i32;

                    if x_oob || y_oob { continue; }
                    let new_p = (new_x as usize, new_y as usize);

                    if visited.contains(&new_p) { continue; }
                    if self.points[0..self.fallen].contains(&new_p) { continue; }

                    let new_cost = f - self.heur(p) + 1;
                    if new_p == (self.v_range, self.h_range) { 
                        return Some(new_cost); 
                    }

                    let new_f = new_cost + self.heur(new_p);
                    
                    // inserting sorted by f in descending order
                    let mut inserted = false;
                    for i in 0..pq.len() {
                        if new_f > pq[i].1 {
                            pq.insert(i, (new_p, new_f));
                            inserted = true;
                            break;
                        }
                    }
                    if !inserted { pq.push((new_p, new_f)); }
                }
             }
        }
        return None;
    }

    fn has_path(&self) -> bool {
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut stack: Vec<(usize, usize)> = vec![(0, 0)];

        while let Some(p) = stack.pop() {
            visited.push(p);
            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    if (x_off == 0) == (y_off == 0) { continue; }

                    let new_x = p.0 as i32 + x_off;
                    let new_y = p.1 as i32 + y_off;

                    let x_oob = new_x < 0 || new_x > self.v_range as i32;
                    let y_oob = new_y < 0 || new_y > self.h_range as i32;

                    if x_oob || y_oob { continue; }
                    let new_p = (new_x as usize, new_y as usize);

                    if visited.contains(&new_p) { continue; }
                    if self.points[0..self.fallen].contains(&new_p) { continue; }

                    if new_p == (self.v_range, self.h_range) { 
                        return true; 
                    }

                    stack.push(new_p);
                }
             }
        }
        return false;
    }

    fn display(&self) {
        let mut s = String::new();

        for y in 0..=self.h_range {
            for x in 0..=self.v_range {
                if self.points[0..self.fallen].contains(&(x,y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        println!("{}", s);
    }
}

fn part1(file_path: &str, h_range: usize, v_range: usize, fallen: usize) -> usize {
    let mut mm = MemoryMaze::from_file(file_path, h_range, v_range);
    mm.fallen = fallen;
    mm.get_shortest_path().expect("MemoryMaze contains no valid path to goal")
}

fn part2(file_path: &str, h_range: usize, v_range: usize) -> (usize, usize) {
    let mut mm = MemoryMaze::from_file(file_path, h_range, v_range);
    let mut lo = 0;
    let mut hi = mm.points.len() - 1;

    while lo < hi {
        println!("{}, {}", lo, hi);
        mm.fallen = (lo + hi) / 2;
        if mm.has_path() {
            lo = mm.fallen + 1;
        } else {
            hi = mm.fallen;
        }
    }
    assert!(lo == hi);
    mm.fallen = lo;
    
    return mm.points[mm.fallen-1];
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Lowest possible score: {} (Duration: {:.2?})",
        part1("src/day18/input.txt", 70, 70, 1024),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Path blocking byte: {:?} (Duration: {:.2?})",
        part2("src/day18/input.txt", 70, 70),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(22, part1("src/day18/test.txt", 6, 6, 12));
    }
    
    #[test]
    fn test_part2() {
        assert_eq!((6, 1), part2("src/day18/test.txt", 6, 6));
    }
}
