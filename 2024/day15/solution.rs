use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Default)]
struct Warehouse {
    height: usize,
    width: usize,
    robot: (usize, usize),
    walls: HashSet<(usize, usize)>,
    boxes: HashSet<(usize, usize)>,
    moves: Vec<(i32, i32)>,
}

impl Warehouse {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        let mut warehouse = Warehouse::default();
        let mut buf_chars = buf.chars();
        let mut x = 0;
        let mut y = 0;

        while let Some(c) = buf_chars.next() {
            match c {
                '#' => { 
                    warehouse.walls.insert((x, y)); 
                    x += 1;
                },
                'O' => {
                    warehouse.boxes.insert((x, y)); 
                    x += 1;
                },
                '@' => {
                    warehouse.robot = (x, y);
                    x += 1;
                },
                '.' => { x += 1; },
                '\n' => {
                    if x == 0 { break; }
                    x = 0;
                    y += 1;
                },
                _ => unreachable!(),
            }
        }

        while let Some(c) = buf_chars.next() {
            match c {
                '^' => { warehouse.moves.push((0,-1)) },
                '<' => { warehouse.moves.push((-1,0)) },
                '>' => { warehouse.moves.push((1, 0)) },
                'v' => { warehouse.moves.push((0, 1)) },
                _ => {},
            }
        }
        warehouse.height = y;
        warehouse.width = buf.lines().next().unwrap().len();
        warehouse
    }

    fn calulate_coordinate_sum(&self) -> usize {
        let mut total = 0;
        for (x, y) in self.boxes.iter() {
            total += x + 100*y; 
        }
        total
    }

    fn make_moves(&mut self) {
        while !self.moves.is_empty() {
            let mv = self.moves.remove(0);
            
            // Should not panic, as robot cannot be on the perimeter.
            let new_x = (self.robot.0 as i32 + mv.0) as usize;
            let new_y = (self.robot.1 as i32 + mv.1) as usize;

            if self.walls.contains(&(new_x, new_y)) {
                continue;
            } else if self.boxes.contains(&(new_x, new_y)) {
                // Recursive moving of boxes
                if !self.move_boxes_recursively((new_x, new_y), mv) { continue; }

                self.boxes.remove(&(new_x, new_y));
            }
            self.robot = (new_x, new_y);
        }
    }

    fn move_boxes_recursively(&mut self, box_pos: (usize, usize), mv: (i32, i32)) -> bool {
        let new_x = (box_pos.0 as i32 + mv.0) as usize;
        let new_y = (box_pos.1 as i32 + mv.1) as usize;

        if self.walls.contains(&(new_x, new_y)) {
            return false;
        } else if self.boxes.contains(&(new_x, new_y)) {
            return self.move_boxes_recursively((new_x, new_y), mv);
        } else {
            self.boxes.insert((new_x, new_y));
            return true;
        }
    }

}

fn part1(file_path: &str) -> usize {
    let mut warehouse = Warehouse::from_file(file_path);
    warehouse.make_moves();
    warehouse.calulate_coordinate_sum()
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Sum of GPS coordinates: {} (Duration: {:.2?})",
        part1("src/day15/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(10092, part1("src/day15/test.txt"));
    }

    #[test]
    fn test_part2() {
        
    }
}
