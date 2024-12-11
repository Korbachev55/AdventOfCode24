use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::RegexSet;


fn main() {
    //let path: &str = "data.txt";
    let path: &str = "small.txt";
    let by_line: Vec<String> = file_to_string_vec(path);

    let horizontal_matches = count_matches(&by_line);
    println!("Horizontal matches: {}", horizontal_matches);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in by_line {
        let as_char_vec: Vec<char> = line.chars().collect();
        matrix.push(as_char_vec);
    }
    
    let transposed = transpose(matrix);
    let string_vec = char_to_string_vec(transposed);
    let vertical_matches = count_matches(&string_vec);
    println!("Vertical matches: {}", vertical_matches);

}

fn file_to_string_vec(path: &str) -> Vec<String> {
    let file: File = File::open(&path).expect("Couldn't open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut by_line: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(_) => by_line.push(line.unwrap()),
            Err(_) => break,
        }
    }

    by_line
}
