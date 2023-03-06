use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut seen_lines: HashSet<&str> = HashSet::new();
    let mut duplicates: HashSet<&str> = HashSet::new();

    for line in lines.iter() {
        if !seen_lines.insert(line) {
            duplicates.insert(line);
        }
    }

    println!("Duplicates:");
    for duplicate in duplicates.iter() {
        println!("{}", duplicate);
    }
}
