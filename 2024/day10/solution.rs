use std::io::Read;
use std::fs::File;
use std::time::Instant;

#[derive(Debug, Default)]
struct TopographicMap {
    nodes: Vec<i32>,
    height: usize,
    width: usize,
}

impl TopographicMap {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        let mut map = TopographicMap::default();

        for c in buf.chars() {
            if c.is_whitespace() { continue; }

            map.nodes.push(c.to_digit(10).unwrap() as i32);
        }
        map.height = buf.lines().collect::<Vec<&str>>().len();
        map.width = buf.lines().next().unwrap().len();
        map
    }

    fn trailheads(&self) -> impl Iterator<Item = usize> + '_ {
        return (0..self.nodes.len())
            .into_iter()
            .filter(|i| self.nodes[*i] == 0);
    }

    fn trailhead_score(&self, idx: usize) -> i32 {
        // BFS without revisiting nodes
        assert!(self.nodes[idx] == 0);

        let offsets: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1,0), (0,-1)];

        let mut score = 0;
        let mut queue = vec![idx];

        while !queue.is_empty() {
            let curr_idx = queue.remove(0);

            if self.nodes[curr_idx] == 9 {
                score = queue.len() + 1;
                break;
            }

            let curr_x = curr_idx % self.width;
            let curr_y = curr_idx / self.width;
            
            for (x_off, y_off) in offsets.iter() {
                let new_x = curr_x as i32 + x_off;
                let new_y = curr_y as i32 + y_off;

                let x_oob = new_x < 0 || new_x >= self.width as i32;
                let y_oob = new_y < 0 || new_y >= self.height as i32;

                if x_oob || y_oob { continue; }

                let new_idx = new_x as usize + (new_y as usize)*self.width;

                if queue.contains(&new_idx) { continue; }

                if self.nodes[new_idx] == self.nodes[curr_idx] + 1 {
                    queue.push(new_idx);
                }
            }
        }
        score as i32
    }

    fn trailhead_rating(&self, idx: usize) -> i32 {
        // BFS with revisiting allowed
        assert!(self.nodes[idx] == 0);

        let offsets: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1,0), (0,-1)];

        let mut score = 0;
        let mut queue = vec![idx];

        while !queue.is_empty() {
            let curr_idx = queue.remove(0);

            if self.nodes[curr_idx] == 9 {
                score += 1;
                continue;
            }

            let curr_x = curr_idx % self.width;
            let curr_y = curr_idx / self.width;
            
            for (x_off, y_off) in offsets.iter() {
                let new_x = curr_x as i32 + x_off;
                let new_y = curr_y as i32 + y_off;

                let x_oob = new_x < 0 || new_x >= self.width as i32;
                let y_oob = new_y < 0 || new_y >= self.height as i32;

                if x_oob || y_oob { continue; }

                let new_idx = new_x as usize + (new_y as usize)*self.width;

                if self.nodes[new_idx] == self.nodes[curr_idx] + 1 {
                    queue.push(new_idx);
                }
            }
        }
        score as i32
    }
}

fn part1(file_path: &str) -> i32 {
    let mut total_scores = 0;
    let map = TopographicMap::from_file(file_path);

    for head in map.trailheads() {
        total_scores += map.trailhead_score(head);
    }

    total_scores
}

fn part2(file_path: &str) -> i32 {
    let mut total_rating = 0;
    let map = TopographicMap::from_file(file_path);

    for head in map.trailheads() {
        total_rating += map.trailhead_rating(head);
    }

    total_rating
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Total score of trailheads: {} (Duration: {:.2?})",
        part1("src/day10/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Total rating of trailheads: {} (Duration: {:.2?})",
        part2("src/day10/input.txt"),
        start.elapsed()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
       assert_eq!(36, part1("src/day10/test.txt"));
    }

    #[test]
    fn test_part2() {
       assert_eq!(81, part2("src/day10/test.txt"));
    }
}
