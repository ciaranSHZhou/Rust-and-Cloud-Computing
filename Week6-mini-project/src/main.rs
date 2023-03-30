//write the function to count the number of words in a file, input is a .txt file, output is the number of words in the file.
//use the function to count the number of words in the file "poem.txt"
use std::fs;
fn main() {
    let filename = "poem.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let words = contents.split_whitespace().count();
    println!("There are {} words in the file {}", words, filename);
}
