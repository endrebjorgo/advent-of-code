use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, Default)]
struct Disk {
    original_map: String,
    layout: Vec<i64>,
}

impl Disk {
    fn from_str(s: &str) -> Self {
        let mut disk = Disk::default();
        disk.original_map = s.trim_end().to_string();

        let mut layout: Vec<i64> = Vec::new();
        let mut id: i64 = 0;
        for (i, c) in disk.original_map.chars().enumerate() {
            let count: usize = c.to_digit(10).unwrap().try_into().unwrap();
            if i % 2 == 0 {
                layout.append(&mut vec![id; count]);
                id += 1;
            } else {
                layout.append(&mut vec![-1; count]);
            }
        }
        disk.layout = layout;
        disk
    }

    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();
        Self::from_str(&buf)
    }

    fn calculate_checksum(&self) -> i64 {
        let mut checksum: i64 = 0;

        for (i, &id) in self.layout.iter().enumerate() {
            if id == -1 { continue; }
            checksum += id * (i as i64);
        }
        checksum
    }

    fn compact(&mut self) {
        let mut i = 0;
        let mut j = self.layout.len() - 1;

        loop {
            while self.layout[i] != -1 {
                i += 1;
            }
            while self.layout[j] == -1 {
                j -= 1;
            }
            if i > j { break; }

            self.layout[i] = self.layout[j];
            self.layout[j] = -1;
        }
    }

    fn compact_chunked(&mut self) {
        let mut chunk_layout = self.generate_chunk_layout();
        assert_ne!(chunk_layout.iter().last().unwrap().0, -1);

        let mut j = chunk_layout.len() - 1;
        let mut current_id = chunk_layout[j].0;

        loop {
            if current_id == 0 { break; }

            // Find index of current chunk we are looking to move.
            while j > 0 {
                if chunk_layout[j].0 == current_id { break; }
                j -= 1;
            }

            for i in 0..j {
                // Look for first available empty chunk which is big enough
                if chunk_layout[i].0 == -1 && 
                    chunk_layout[i].1 >= chunk_layout[j].1 
                {
                    let d: usize = chunk_layout[i].1
                        .checked_sub(chunk_layout[j].1)
                        .unwrap_or(0);
                    chunk_layout[i] = chunk_layout[j];
                    chunk_layout[j] = (-1, chunk_layout[i].1);

                    if d == 0 { break; }

                    // Reinsert empty chunk or add it to existing succeding empty chunk
                    if chunk_layout[i+1].0 == -1 {
                        chunk_layout[i+1].1 += d;
                    } else {
                        chunk_layout.insert(i+1, (-1, d));
                        j += 1;
                    }
                    break;
                }
            }
            current_id -= 1;
        }

        let mut new_layout = Vec::new();
        for (id, length) in chunk_layout.iter() {
            new_layout.append(&mut vec![*id; *length]);
        }
        self.layout = new_layout;
    }

    fn generate_chunk_layout(&self) -> Vec<(i64, usize)> {
        let mut chunk_layout: Vec<(i64, usize)> = Vec::new();
        let mut id: i64 = 0;
        for (i, c) in self.original_map.trim_end().chars().enumerate() {
            let count: usize = c.to_digit(10).unwrap().try_into().unwrap();
            if i % 2 == 0 {
                chunk_layout.push((id, count));
                id += 1;
            } else {
                chunk_layout.push((-1, count));
            }
        }
        chunk_layout
    }
}

fn part1(file_path: &str) -> i64 {
    let mut disk = Disk::from_file(file_path);
    disk.compact();
    disk.calculate_checksum()
}

fn part2(file_path: &str) -> i64 {
    let mut disk = Disk::from_file(file_path);
    disk.compact_chunked();
    disk.calculate_checksum()
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Resulting filesystem checksum: {} (Duration: {:.2?})",
        part1("src/day09/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Resulting filesystem checksum (chunked): {} (Duration: {:.2?})",
        part2("src/day09/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1928, part1("src/day09/test.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, part2("src/day09/test.txt"));
    }
}
