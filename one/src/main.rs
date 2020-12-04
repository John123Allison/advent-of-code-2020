use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() {
    solution_one();
    solution_two();
}

fn solution_one() {
    let mut solution_one: i32 = 0;
    let mut buff: HashMap<i32, i32> = HashMap::new();

    // Read file contents
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    // Insert each value into the hash map as the key to the value which
    // is the complement to 2021
    for line in lines {
        let line_string = line.unwrap().to_string();
        let line_int: i32 = line_string.parse().unwrap();
        buff.insert(line_int, 2020 - line_int);
    }

    // Find key that matches the value, lazily
    for (value, complement) in &buff {
        if buff.contains_key(complement) {
            solution_one = value * complement;
        }
    }

    println!("Solution one: {}", solution_one);
}

fn solution_two() {
    let mut solution_two: i32 = 0;
    let mut buff: Vec<i32> = Vec::new();

    // Read file contents
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let line_string = line.unwrap().to_string();
        let line_int: i32 = line_string.parse().unwrap();
        buff.push(line_int);
    }

    // Do the actual math - gonna do it like an idiot because I am an idiot
    for val in &buff {
        for val_two in &buff {
            for val_three in &buff {
                if val + val_two + val_three == 2020 {
                    solution_two = val * val_two * val_three;
                }
            }
        }
    }

    println!("Solution two: {}", solution_two);
}
