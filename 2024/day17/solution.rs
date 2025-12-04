use std::io::Read;
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
struct Computer {
    ip: usize,
    program: Vec<usize>,
    a_reg: usize,
    b_reg: usize,
    c_reg: usize,
}

impl Computer {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        let mut buf_lines = buf.lines();

        let a_reg_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;
        let b_reg_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;
        let c_reg_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;

        let a_reg = usize::from_str_radix(a_reg_str, 10).unwrap();
        let b_reg = usize::from_str_radix(b_reg_str, 10).unwrap();
        let c_reg = usize::from_str_radix(c_reg_str, 10).unwrap();

        assert!(buf_lines.next() == Some(""));

        let program_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;
        let program: Vec<usize> = program_str.split(",")
            .map(|e| usize::from_str_radix(e, 10).unwrap()).collect();
        
        assert!(buf_lines.next() == None);

        Self { ip: 0, program, a_reg, b_reg, c_reg }
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            x if x <= 3 => x,
            4 => self.a_reg,
            5 => self.b_reg,
            6 => self.c_reg,
            _ => unreachable!(),
        }
    }

    fn execute_op(&mut self, output: &mut Vec<usize>) {
        let optype = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        match optype {
            0 => {
                self.a_reg /= usize::pow(2, self.combo(operand) as u32);
            },
            1 => {
                self.b_reg ^= operand;
            },
            2 => {
                self.b_reg = self.combo(operand) % 8;
            },
            3 => {
                if self.a_reg != 0 {
                    self.ip = operand;
                    return;
                }
            },
            4 => {
                self.b_reg ^= self.c_reg;
            },
            5 => {
                output.push(self.combo(operand) % 8);
            },
            6 => {
                self.b_reg = self.a_reg / usize::pow(2, self.combo(operand) as u32);
            },
            7 => {
                self.c_reg = self.a_reg / usize::pow(2, self.combo(operand) as u32);
            },
            _ => unreachable!(),
        }
        self.ip += 2;
    }

    fn execute_program(&mut self) -> Vec<usize> {
        let mut output: Vec<usize> = Vec::new();

        while self.ip < self.program.len() {
            self.execute_op(&mut output);
        }
        self.ip = 0;
        output
    }


    fn get_fixed_point(&mut self) -> usize {
        let mut a_reg = 0;

        for _ in 0..100 {
            self.a_reg = a_reg;
            let op = self.execute_program();
            println!("{:?}", op);
            a_reg += 1;
        }
        return 0;
        /*
        let mut a_reg = 0;

        self.a_reg = a_reg;
        let initial_output = self.execute_program();

        loop {
            a_reg += 1;
            self.a_reg = a_reg;
            if self.execute_program() != initial_output { break; }
        }

        println!("{:?}", a_reg);
        self.a_reg = a_reg;
        let curr_program = self.execute_program();
        let mut start: usize = 0;
        for i in 0..curr_program.len() {
            start += curr_program[i] << (3 * (i + 1));
        }

        let mut goal: usize = 0;
        for i in 0..self.program.len() {
            goal += self.program[i] << (3 * (i + 1));
        }

        println!("{:?}", initial_output);
        println!("{:?} -> {}", curr_program, start);
        println!("{:?} -> {}", self.program, goal);

        let new_a_reg = a_reg + goal - start;

        self.a_reg = new_a_reg;
        let res = self.execute_program(); 
        println!("{:?}", res);
        return new_a_reg;
        */
    }
}

fn part1(file_path: &str) -> Vec<usize> {
    let mut c = Computer::from_file(file_path);
    c.execute_program()
}

fn part2(file_path: &str) -> usize {
    let mut c = Computer::from_file(file_path);
    c.get_fixed_point()
}

fn main() {
    let mut start = Instant::now();

    let output = part1("src/day17/input.txt").iter()
        .map(|e| e.to_string()).collect::<Vec<String>>().join(",");
    println!(
        "Output: {} (Duration: {:.2?})",
        output,
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Fix-point of A register: {} (Duration: {:.2?})",
        part2("src/day17/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        //let res: Vec<usize> = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        //assert_eq!(res, part1("src/day17/test1.txt"));
    }

    #[test]
    fn test_part2() {
        //assert_eq!(117440, part2("src/day17/test2.txt"));
    }
}
