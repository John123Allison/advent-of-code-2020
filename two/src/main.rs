use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    solution_one();
}

fn solution_one() {
    let mut num_valid = 0;

    // Read file contents
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    // Iterate over each line and do the math, so to speak
    for line in lines {
        // Get each line as a collection of tokens
        let line_string = line.unwrap().to_string();
        let split: Vec<&str> = line_string.split(" ").collect();
        let policy = split[0];
        let token = split[1];
        let pass = split[2];

        // Check if password matches policy
        let policy_split: Vec<&str> = policy.split("-").collect();
        let pass_split: Vec<char> = pass.chars().collect();
        let min: i32 = policy_split[0].parse().unwrap();
        let max: i32 = policy_split[1].parse().unwrap();
        let mut char_instances = 0;

        for character in pass_split {
            if character == token.chars().nth(0).unwrap() {
                char_instances += 1;
            }
        }

        if char_instances >= min && char_instances <= max {
            num_valid += 1;
        }
    }

    println!("Solution one: {}", num_valid);
}
