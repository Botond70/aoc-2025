use std::{cmp::Ordering, path};
struct IdRange {
    pub start: u128,
    pub end: u128,
}

impl IdRange {
    fn new(start: u128, end: u128) -> Self {
        IdRange { start, end }
    }

    fn contains(&self, id: u128) -> bool {
        id >= self.start && id <= self.end
    }
}

struct invalidIdFinder {
    ids: Vec<IdRange>,
    invalid_id_sum: u128,
}
fn is_even_length(id: u128) -> bool {
    let length = id.to_string().len();
    length % 2 == 0
}
impl invalidIdFinder {
    fn new(ids: Vec<IdRange>) -> Self {
        invalidIdFinder {
            ids,
            invalid_id_sum: 0,
        }
    }

    fn evaluate(&mut self) {
        self.invalid_id_sum = 0;
        for idr in self.ids.iter() {
            for id in idr.start..idr.end + 1 {
                if is_even_length(id) {
                    let idlength = id.to_string().len();
                    let s = id.to_string();
                    let half = idlength / 2;
                    let left: i128 = s[..half].parse().expect("left half not numeric");
                    let right: i128 = s[half..].parse().expect("right half not numeric");
                    /*
                    println!(
                        "ID {} split into top half: {} and bottom half: {}",
                        id, left, right
                    );*/
                    if left == right {
                        //println!("Invalid ID found: {}", id);
                        self.invalid_id_sum += id;
                    }
                }
            }
        }
    }

    fn evaluate2(&mut self) {
        self.invalid_id_sum = 0;
        for idr in self.ids.iter() {
            for id in idr.start..idr.end + 1 {
                let idlength = id.to_string().len();
                for parts in 2..=idlength {
                    if idlength % parts == 0 {
                        let s = id.to_string();
                        let part_length = idlength / parts;
                        let mut all_equal = true;
                        let first_part: i128 = s[0..part_length].parse().expect("part not numeric");
                        for p in 1..parts {
                            let start_index = p * part_length;
                            let end_index = start_index + part_length;
                            let current_part: i128 =
                                s[start_index..end_index].parse().expect("part not numeric");
                            if current_part != first_part {
                                all_equal = false;
                                break;
                            }
                        }
                        if all_equal {
                            //println!("Invalid ID found: {}", id);
                            self.invalid_id_sum += id;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn print_invalid_ids(&self) {
        println!("Sum of invalid IDs: {}", self.invalid_id_sum);
    }
}

fn main() {
    let file_path = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let contents = std::fs::read_to_string(file_path).expect("Failed to read input.txt");
    let mut id_ranges = Vec::<IdRange>::new();
    for line in contents.lines() {
        let id_list = line.split(',');
        for ranges in id_list {
            let bounds: Vec<&str> = ranges.split('-').collect();
            let start: u128 = bounds[0].parse().expect("start not numeric");
            let end: u128 = bounds[1].parse().expect("end not numeric");
            id_ranges.push(IdRange::new(start, end));
        }
    }

    let mut finder = invalidIdFinder::new(id_ranges);
    finder.evaluate();
    finder.print_invalid_ids();

    finder.evaluate2();
    finder.print_invalid_ids();
}
