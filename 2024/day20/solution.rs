use std::io::Read;
use std::fs::File;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct Racetrack {
    walls: HashSet<(usize, usize)>, 
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
    cheat: Option<[(usize, usize); 2]>,
}

impl Racetrack {
    fn from_file(file_path: &str) -> Self {
        let mut buf = Vec::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_end(&mut buf).unwrap();

        let mut result = Self::default();
        let mut x: usize = 0;
        let mut y: usize = 0;

        for &c in buf.iter() {
            if c == b'\n' {
                x = 0;
                y += 1;
                continue;
            }
            match c {
                b'#' => { result.walls.insert((x, y)); },
                b'S' => { result.start = (x, y); },
                b'E' => { result.end = (x, y); },
                b'.' => {},
                _ => unimplemented!(),
            }
            x += 1;
        }
        result.width = buf.iter().position(|&e| e == b'\n').unwrap();
        result.height = y;
        result 
    }

    fn apply_cheat(&mut self, p1: (usize, usize), p2: (usize, usize)) {
        self.cheat = Some([p1, p2]);
    }

    fn heuristic(&self, p: (usize, usize)) -> usize {
        let dx = (self.end.0 as i32 - p.0 as i32).abs() as usize;
        let dy = (self.end.1 as i32 - p.1 as i32).abs() as usize;
        return dx + dy;
    }

    fn a_star(&self) -> Option<usize> {
        let mut priority_q: Vec<((usize, usize), usize)> 
            = vec![(self.start, self.heuristic(self.start))];
        let mut visited: Vec<(usize, usize)> = Vec::new();

        let mut walls: HashSet<(usize, usize)> = self.walls.clone();

        if let Some(c) = self.cheat {
            walls.retain(|&e| e != c[0] && e != c[1]);
        }

        while let Some((p, f)) = priority_q.pop() {
            visited.push(p);
            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    if (x_off == 0) == (y_off == 0) { continue; }
                    
                    let new_x = p.0 as i32 + x_off;
                    let new_y = p.1 as i32 + y_off;

                    let x_oob = new_x < 0 || new_x >= self.width as i32;
                    let y_oob = new_y < 0 || new_y >= self.height as i32;
                    if x_oob || y_oob { continue; }

                    let new_p = (new_x as usize, new_y as usize);

                    if visited.contains(&new_p) { continue; }
                    if walls.contains(&new_p) { continue; }

                    let new_cost = f - self.heuristic(p) + 1;
                    if new_p == self.end { 
                        return Some(new_cost); 
                    }

                    let new_f = new_cost + self.heuristic(new_p);
                    
                    // inserting sorted by f in descending order
                    let mut inserted = false;
                    for i in 0..priority_q.len() {
                        if new_f > priority_q[i].1 {
                            priority_q.insert(i, (new_p, new_f));
                            inserted = true;
                            break;
                        }
                    }
                    if !inserted { priority_q.push((new_p, new_f)); }
                }
            }
        }
        return None;
    }

    fn get_to_costs(&self) -> HashMap<(usize, usize), usize> {
        let mut priority_q: Vec<((usize, usize), usize)> 
            = vec![(self.start, self.heuristic(self.start))];
        let mut visited: Vec<(usize, usize)> = Vec::new();

        let mut walls: HashSet<(usize, usize)> = self.walls.clone();

        if let Some(c) = self.cheat {
            walls.retain(|&e| e != c[0] && e != c[1]);
        }

        while let Some((p, f)) = priority_q.pop() {
            visited.push(p);
            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    if (x_off == 0) == (y_off == 0) { continue; }
                    
                    let new_x = p.0 as i32 + x_off;
                    let new_y = p.1 as i32 + y_off;

                    let x_oob = new_x < 0 || new_x >= self.width as i32;
                    let y_oob = new_y < 0 || new_y >= self.height as i32;
                    if x_oob || y_oob { continue; }

                    let new_p = (new_x as usize, new_y as usize);

                    if visited.contains(&new_p) { continue; }
                    if walls.contains(&new_p) { continue; }

                    let new_cost = f - self.heuristic(p) + 1;
                    if new_p == self.end { 
                        return Some(new_cost); 
                    }

                    let new_f = new_cost + self.heuristic(new_p);
                    
                    // inserting sorted by f in descending order
                    let mut inserted = false;
                    for i in 0..priority_q.len() {
                        if new_f > priority_q[i].1 {
                            priority_q.insert(i, (new_p, new_f));
                            inserted = true;
                            break;
                        }
                    }
                    if !inserted { priority_q.push((new_p, new_f)); }
                }
            }
        }
        return None;
    }
}

fn part1(file_path: &str) -> usize {
    let mut racetrack = Racetrack::from_file(file_path);
    let initial_time = racetrack.a_star().unwrap();
    let mut count = 0;
    let walls = racetrack.walls.clone();
    let n = walls.len();

    for (i, &p1) in walls.iter().enumerate() {
        println!("{}/{}", i, n);
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                if (x_off == 0) == (y_off == 0) { continue; }
                
                let x2 = p1.0 as i32 + x_off;
                let y2 = p1.1 as i32 + y_off;

                let x_oob = x2 < 0 || x2 >= racetrack.width as i32;
                let y_oob = y2 < 0 || y2 >= racetrack.height as i32;
                if x_oob || y_oob { continue; }

                let p2 = (x2 as usize, y2 as usize);

                racetrack.apply_cheat(p1, p2); 
                if let Some(t) = racetrack.a_star() {
                    if t + 100 < initial_time { count += 1 }
                }
            }
        }
    }
    return count;
}

fn main() {
    let x = part1("src/day20/input.txt");
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        
    }
}
