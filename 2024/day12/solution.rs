use std::io::Read;
use std::fs::File;
use std::collections::HashSet;
use std::time::Instant;

static DIRS: [(i32, i32); 4] = [(0,-1), (1, 0), (0, 1), (-1,0)];

#[derive(Default, Debug)]
struct Farm {
    plots: Vec<char>,
    height: usize,
    width: usize,
}

impl Farm {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();
        
        let mut farm = Self::default();

        for line in buf.lines() {
            for c in line.chars() {
                farm.plots.push(c);
            }
        }
        farm.height = buf.lines().collect::<Vec<&str>>().len();
        farm.width = buf.lines().next().unwrap().len();
        farm
    }

    fn calculate_fence_price(&self) -> i32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut total_price = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if visited.contains(&(x,y)) {
                    continue;
                }
                total_price += self.calculate_region_price(x, y, &mut visited);
            }
        }
        total_price
    }

    fn calculate_region_price(&self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i32 {
        let mut stack = vec![(x,y)];
        let mut area = 0;
        let mut perimeter = 0;

        let curr_char = self.plots[x + y*self.width];
        
        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            visited.insert(curr);
            area += 1;

            for dir in DIRS.iter() {
                let new_x = curr.0 as i32 + dir.0;
                let new_y = curr.1 as i32 + dir.1;

                let new_x_oob = new_x < 0 || new_x >= self.width as i32;
                let new_y_oob = new_y < 0 || new_y >= self.height as i32;

                if new_x_oob || new_y_oob {
                    perimeter += 1;
                    continue;
                } 

                let new_idx = new_x as usize + (new_y as usize)*self.width;
                if self.plots[new_idx] != curr_char {
                    perimeter += 1;
                    continue;
                }

                let new_pos = (new_x as usize, new_y as usize);
                if !stack.contains(&new_pos) && !visited.contains(&new_pos) {
                    stack.push(new_pos);
                }
            }
        }
        return area*perimeter;
    }

    fn calculate_fence_bulk_price(&self) -> i32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut total_price = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if visited.contains(&(x,y)) {
                    continue;
                }
                total_price += self.calculate_region_bulk_price(x, y, &mut visited);
            }
        }
        total_price
    }

    fn calculate_region_bulk_price(&self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i32 {
        let mut stack = vec![(x,y)];
        // Ordered top, left, right, bottom
        let mut perimeters: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 4];
        let mut area = 0;

        let curr_char = self.plots[x + y*self.width];
        
        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            visited.insert(curr);
            area += 1;

            for dir in DIRS.iter() {
                let new_x = curr.0 as i32 + dir.0;
                let new_y = curr.1 as i32 + dir.1;

                let new_x_oob = new_x < 0 || new_x >= self.width as i32;
                let new_y_oob = new_y < 0 || new_y >= self.height as i32;

                if new_x_oob || new_y_oob {
                    match dir {
                        (0,-1) => { perimeters[0].push(curr); },
                        (-1,0) => { perimeters[1].push(curr); },
                        (1, 0) => { perimeters[2].push(curr); },
                        (0, 1) => { perimeters[3].push(curr); },
                        _ => unreachable!(),
                    }
                    continue;
                } 

                let new_idx = new_x as usize + (new_y as usize)*self.width;
                if self.plots[new_idx] != curr_char {
                    match dir {
                        (0,-1) => { perimeters[0].push(curr); },
                        (-1,0) => { perimeters[1].push(curr); },
                        (1, 0) => { perimeters[2].push(curr); },
                        (0, 1) => { perimeters[3].push(curr); },
                        _ => unreachable!(),
                    }
                    continue;
                }

                let new_pos = (new_x as usize, new_y as usize);
                if !stack.contains(&new_pos) && !visited.contains(&new_pos) {
                    stack.push(new_pos);
                }
            }
        }
        let walls = Self::count_walls(perimeters);
        return area*walls;
    }

    fn count_walls(perimeters: Vec<Vec<(usize, usize)>>) -> i32 {
        let mut t_peri = perimeters[0].clone();
        let mut l_peri = perimeters[1].clone();
        let mut r_peri = perimeters[2].clone();
        let mut b_peri = perimeters[3].clone();
        let mut walls = 0;

        let mut offset: usize;

        while !t_peri.is_empty() {
            let curr = t_peri.pop().unwrap();

            offset = 1;
            loop {
                let curr_neigh = (curr.0 + offset, curr.1);
                if t_peri.contains(&curr_neigh) {
                    t_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }

            offset = 1;
            loop {
                if offset > curr.0 { break; }

                let curr_neigh = (curr.0 - offset, curr.1);
                if t_peri.contains(&curr_neigh) {
                    t_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }
            walls += 1;
        }

        while !l_peri.is_empty() {
            let curr = l_peri.pop().unwrap();

            offset = 1;
            loop {
                let curr_neigh = (curr.0, curr.1 + offset);
                if l_peri.contains(&curr_neigh) {
                    l_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }

            offset = 1;
            loop {
                if offset > curr.1 { break; }

                let curr_neigh = (curr.0, curr.1 - offset);
                if l_peri.contains(&curr_neigh) {
                    l_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }
            walls += 1;
        }

        while !r_peri.is_empty() {
            let curr = r_peri.pop().unwrap();

            offset = 1;
            loop {
                let curr_neigh = (curr.0, curr.1 + offset);
                if r_peri.contains(&curr_neigh) {
                    r_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }

            offset = 1;
            loop {
                if offset > curr.1 { break; }


                let curr_neigh = (curr.0, curr.1 - offset);
                if r_peri.contains(&curr_neigh) {
                    r_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }
            walls += 1;
        }

        while !b_peri.is_empty() {
            let curr = b_peri.pop().unwrap();

            offset = 1;
            loop {
                let curr_neigh = (curr.0 + offset, curr.1);
                if b_peri.contains(&curr_neigh) {
                    b_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }

            offset = 1;
            loop {
                if offset > curr.0 { break; }

                let curr_neigh = (curr.0 - offset, curr.1);
                if b_peri.contains(&curr_neigh) {
                    b_peri.retain(|&e| e != curr_neigh);
                    offset += 1;
                } else {
                    break;
                }
            }
            walls += 1;
        }
        return walls;
    }
}

fn part1(file_path: &str) -> i32 {
    let farm = Farm::from_file(file_path);
    farm.calculate_fence_price()
}

fn part2(file_path: &str) -> i32 {
    let farm = Farm::from_file(file_path);
    farm.calculate_fence_bulk_price()
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Total price of fence: {} (Duration: {:.2?})",
        part1("src/day12/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Total price of fence w/ bulk discount: {} (Duration: {:.2?})",
        part2("src/day12/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1930, part1("src/day12/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1206, part2("src/day12/test.txt"));
    }
}
