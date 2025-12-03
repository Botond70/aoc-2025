use std::path;

struct BatteryBank {
    batteryList: String,
    joltage: u128,
}

impl BatteryBank {
    fn new(batteryList: String) -> BatteryBank {
        BatteryBank {
            batteryList,
            joltage: 0,
        }
    }

    fn calc_joltage(&mut self) {
        self.joltage = 0;
        let mut batteries: Vec<u8> = Vec::new();
        for c in self.batteryList.chars() {
            batteries.push(c.to_digit(10).unwrap() as u8);
        }

        let mut first_digit_index = 0u32;
        let mut first_digit = 0u8;
        for b in 0..batteries.len() - 1 {
            if batteries[b] > first_digit {
                first_digit = batteries[b];
                first_digit_index = b as u32;
            }
        }
        let mut second_digit = 0u8;
        for b in (first_digit_index + 1) as usize..batteries.len() {
            if batteries[b] > second_digit {
                second_digit = batteries[b];
            }
        }

        self.joltage = (first_digit as u128) * 10 + (second_digit as u128);
    }

    fn calc_joltage2(&mut self) {
        self.joltage = 0;
        let mut batteries: Vec<u8> = Vec::new();
        let mut remaining_batteries = 12usize;
        for c in self.batteryList.chars() {
            batteries.push(c.to_digit(10).unwrap() as u8);
        }
        let mut digits = Vec::<u8>::new();
        let mut boundaries = Vec::<usize>::new();
        boundaries.push(0);
        while remaining_batteries > 0 {
            let mut working_digit = 0u8;
            let mut working_index = boundaries[boundaries.len() - 1] as usize;
            let boundary_index = working_index;
            println!(
                "Debug: remaining batteries {}, boundary index {}",
                remaining_batteries, boundary_index
            );
            for b in boundary_index..batteries.len() - remaining_batteries + 1 {
                if batteries[b] > working_digit {
                    working_digit = batteries[b];
                    working_index = b;
                    if working_digit == 9 {
                        break;
                    }
                }
            }
            digits.push(working_digit);
            boundaries.push(working_index + 1);
            remaining_batteries -= 1;
        }
        for d in digits.iter() {
            self.joltage = (self.joltage * 10) + d.clone() as u128;
        }
    }

    fn print_joltage(&self) {
        println!("Calculated joltage is {}", self.joltage);
    }
    fn get_joltage(&self) -> u128 {
        self.joltage
    }
}

fn main() {
    let file_path = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let contents = std::fs::read_to_string(file_path).expect("Failed to read input.txt");

    let mut banks = Vec::<BatteryBank>::new();
    for line in contents.lines() {
        let mut bank = BatteryBank::new(line.to_string());
        bank.calc_joltage();
        banks.push(bank);
    }
    let mut total_joltage = 0u128;
    for b in banks.iter() {
        b.print_joltage();
        total_joltage += b.get_joltage();
    }
    println!("Total joltage is {}", total_joltage);

    let mut total_joltage2 = 0u128;
    for b in banks.iter_mut() {
        b.calc_joltage2();
        println!("Recalculated joltage is {}", b.get_joltage());
        total_joltage2 += b.get_joltage();
    }
    println!("Total joltage (method 2) is {}", total_joltage2);
}
