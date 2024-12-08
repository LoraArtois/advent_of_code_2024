use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::path::Path;

fn main() {
    similarity();
}

fn similarity() {
    let mut list0 = Vec::new();
    let mut list1 = Vec::new();
    let mut result: i64 = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let output: Vec<&str> = regex::Regex::new(" +")
                .unwrap()
                .split(line.as_str())
                .collect();
            list0.push(output[0].parse::<i64>().unwrap());
            list1.push(output[1].parse::<i64>().unwrap());
        }
    }

    for i in 0..list0.len() {
        let appearances: i64 = list1.iter().filter(|&x| *x == list0[i]).count().try_into().unwrap();
        let similarity = list0[i] * appearances;
        result = result.add(similarity);
    }
    println!("{}", result);
}

fn diff() {
    let mut list0 = Vec::new();
    let mut list1 = Vec::new();
    let mut result: i64 = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let output: Vec<&str> = regex::Regex::new(" +")
                .unwrap()
                .split(line.as_str())
                .collect();
            list0.push(output[0].parse::<i64>().unwrap());
            list1.push(output[1].parse::<i64>().unwrap());
        }
    }
    list0.sort();
    list1.sort();

    for i in 0..list0.len() {
        result += (list0[i] - list1[i]).abs();
    }
    println!("{}", result);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
