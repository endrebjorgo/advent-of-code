use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;


#[derive(Debug)]
struct Robot {
    pos: (usize, usize),
    v: (i32, i32),
}

impl Robot {
    fn new(pos: (usize, usize), v: (i32, i32)) -> Self {
        Self { pos, v }
    }
}

#[derive(Debug)]
struct Bathroom {
    height: usize, 
    width: usize,
    robots: Vec<Robot>
}

impl Bathroom {
    fn from_file(file_path: &str, height: usize, width: usize) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();
        
        let mut robots = Vec::new();
        for line in buf.lines() {
            let (rest, v_str) = line.split_once(" v=").unwrap();
            let pos_str = &rest[2..];

            let mut pos_it = pos_str.split(",")
                .map(|e| usize::from_str_radix(e, 10).unwrap());
            let mut v_it = v_str.split(",")
                .map(|e| i32::from_str_radix(e, 10).unwrap());

            let pos = (pos_it.next().unwrap(), pos_it.next().unwrap());
            let v = (v_it.next().unwrap(), v_it.next().unwrap());
            robots.push(Robot::new(pos, v));
        }
        Self { height, width, robots } 
    }

    fn step_n_times(&mut self, n: i32) {
        for robot in self.robots.iter_mut() {
            assert!(n > 0);

            let mut new_x = (robot.pos.0 as i32 + (robot.v.0*n)) % self.width as i32;
            let mut new_y = (robot.pos.1 as i32 + (robot.v.1*n)) % self.height as i32;

            if new_x < 0 { new_x += self.width as i32; }
            if new_y < 0 { new_y += self.height as i32; }

            robot.pos.0 = new_x as usize;
            robot.pos.1 = new_y as usize;
        }
    }

    fn calculate_safety_factor(&self) -> i32 {
        let mut counts: [i32; 4] = [0, 0, 0, 0];
        for rob in self.robots.iter() {
            if rob.pos.0 * 2 + 1 == self.width ||
               rob.pos.1 * 2 + 1 == self.height {
                continue;
            }
            let a = rob.pos.0 * 2 / self.width;
            let b = rob.pos.1 * 2 / self.height;
            counts[a + 2*b] += 1;
        }
        println!("{:?}", counts);
        return counts[0] * counts[1] * counts[2] * counts[3];
    }

    fn render_robots(&self) {
        let mut canvas = vec![vec![' '; self.width]; self.height];
        for robot in self.robots.iter() {
            canvas[robot.pos.1][robot.pos.0] = '#';
        }
        for v in canvas.iter() {
            let line: String = v.iter().collect();
            println!("{}", line);
        }
    }

    fn contains_picture(&self) -> bool {
        // Guessing that the robots are not on top of eachother in the picture.
        // This turned out to be right :)
        let mut visited = HashSet::new();
        for robot in self.robots.iter() {
            if !visited.insert(robot.pos) {
                return false;
            }
        }
        return true;
    }
}

fn part1(file_path: &str, height: usize, width: usize) -> i32 {
    let mut bathroom = Bathroom::from_file(file_path, height, width);
    bathroom.step_n_times(100);
    bathroom.calculate_safety_factor()
}

fn part2(file_path: &str, height: usize, width: usize) -> i32 {
    let mut bathroom = Bathroom::from_file(file_path, height, width);
    let mut seconds = 1;
    loop {
        bathroom.step_n_times(1);
        if bathroom.contains_picture() {
            bathroom.render_robots();
            break;
        }
        seconds += 1;
    }
    seconds
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Safety factor after 100 steps: {} (Duration: {:.2?})",
        part1("src/day14/input.txt", 103, 101),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Seconds until easter egg: {} (Duration: {:.2?})",
        part2("src/day14/input.txt", 103, 101),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(12, part1("src/day14/test.txt", 7, 11));
    }
}
