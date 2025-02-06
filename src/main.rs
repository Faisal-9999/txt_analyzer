use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

fn main() {
    
    let start = Instant::now();

    let mut file_path : String = "words.txt".to_string();

    if !Path::new(&file_path.trim()).exists() {
        println!("Error Occurred File Doensn't Exist");
        std::process::exit(0);
    }

    let file = File::open(&file_path.trim()).unwrap();

    let reader = io::BufReader::new(file);

    let mut words = HashMap::<String, u32>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut temp = String::new();
        for character in line.chars() {
            if character == ' ' || character == ',' || character == '.' {
                *words.entry(temp).or_insert(0) += 1;
                temp = String::new();
            }
            else {
                temp.push(character);
            }
        }
    }

    for (key, value) in words {
        println!("WORD: {}, VALUE: {}", key, value);
    }

    let duration = start.elapsed();

    println!("TIME TAKEN : {:?}", duration);

}