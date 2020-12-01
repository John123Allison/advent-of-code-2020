use std::{collections::HashMap, io::BufRead, io::BufReader, fs::File};

fn main() {
    let mut solution: i32 = 0;
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
        buff.insert(
            line_int,
            2020 - line_int
        );
    }

    // Find key that matches the value, lazily
    for (value, complement) in &buff {
        if buff.contains_key(complement) {
            solution = value * complement;
        }
    }

    println!("{}", solution);
}
