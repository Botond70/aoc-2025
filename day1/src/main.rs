use std::{path, vec::Vec};

fn modulo(a: i16, b: i16) -> i16 {
    ((a % b) + b) % b
}
struct dialSolver {
    input: Vec<i16>,
    position: i16,
    number_of_zeros: u16,
}

impl dialSolver {
    fn new(input: Vec<i16>) -> Self {
        dialSolver {
            input,
            position: 50,
            number_of_zeros: 0,
        }
    }

    fn evaluate(&mut self) {
        self.number_of_zeros = 0;
        self.position = 50;
        for i in self.input.iter() {
            print!("Moving dial by {} from position {} to ", i, self.position);
            self.position += i;
            if self.position > 99 {
                self.position = modulo(self.position, 100);
            }
            if self.position < 0 {
                self.position = modulo(self.position, 100);
            }
            if self.position == 0 {
                self.number_of_zeros += 1;
            }
            print!("{}.\n", self.position);
        }
    }
    fn evaluate2(&mut self) {
        self.number_of_zeros = 0;
        self.position = 50;
        for i in self.input.iter() {
            let was_zero = self.position == 0;
            self.position += i;
            let mut zerospassed = 0;
            if self.position == 0 {
                self.number_of_zeros += 1;
                zerospassed += 1;
                //println!("Debug: position is zero");
            }
            if self.position > 99 {
                let newpostion = modulo(self.position, 100);
                let remainder = self.position % 100;
                let diff = self.position - remainder;
                zerospassed = (diff / 100).abs();
                self.number_of_zeros += (zerospassed) as u16;
                self.position = newpostion;
                //println!("Debug: position after >99: {}", self.position);
            }
            if self.position < 0 {
                let newpostion = modulo(self.position, 100);
                let remainder = self.position % 100;
                let diff = self.position - remainder;
                zerospassed = (diff / 100).abs() + 1;
                if was_zero {
                    zerospassed -= 1;
                }
                self.number_of_zeros += (zerospassed) as u16;
                self.position = newpostion;
                //println!("Debug: position after <0: {}", self.position);
            }
            /*
            print!(
                "Moving dial by {} to position {} passing zero ",
                i, self.position
            );
            print!("{} times.\n", zerospassed);*/
        }
    }

    fn printzeros(&self) {
        println!("Number of zeros: {}", self.number_of_zeros);
    }
}

pub fn main() {
    //part 1
    let path = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let contents = std::fs::read_to_string(path).expect("Failed to read input.txt");
    let mut diffs = Vec::new();
    for line in contents.lines() {
        if line[0..1].eq("R") {
            let number: i16 = line[1..].parse().unwrap();
            diffs.push(number);
        } else {
            let number: i16 = line[1..].parse().unwrap();
            diffs.push(-number);
        }
    }
    println!("Read {} diffs", diffs.len());
    let mut solver = dialSolver::new(diffs);
    solver.evaluate();
    solver.printzeros();
    //part 2
    solver.evaluate2();
    solver.printzeros();
}
