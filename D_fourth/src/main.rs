use std::fs::File;
use std::io::{BufReader, BufRead};
//use regex::RegexSet;
//mod readfile; // Raikku

fn main() {
    // Raikku
    //let data = readfile::readfile("small.txt");
    //println!("data: {:?}", data[0]);

    let path: &str = "small.txt";
    let by_line: Vec<String> = file_to_string_vec(path);
    let mut collector: usize = 0;

    // Maken kaunis matriisiks luku
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in by_line {
        let as_char_vec: Vec<char> = line.chars().collect();
        matrix.push(as_char_vec);
    }

    // Get the transposed matrix
    let mut matrix_transposed: Vec<Vec<char>> = matrix.clone();
    matrix_transposed = transpose(matrix_transposed);
    
    // Print
    //println!("Count horizontal: {}", count_horizontal(&matrix[0]));



    // Horizontal
    for line in matrix {
        println!("Count horizontal: {}", count_horizontal(&line));
        collector += count_horizontal(&line);
    }
    println!("Horizontal: {}", collector);
    

    // Vertical
    for line in matrix_transposed {
        println!("Count horizontal: {}", count_horizontal(&line));
        collector += count_horizontal(&line);
    }
    println!("Vertical: {}", collector);




    println!("Collector: {}", collector);

}



fn count_horizontal(_data: &Vec<char>) -> usize {
    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let samx: Vec<char> = vec!['S', 'A', 'M', 'X'];
    //println!("data: {:?}", _data);
    let forward: usize = _data
        .windows(4)
        .filter(|window| window == &xmas.as_slice())
        .count();
    let backtward: usize = _data
        .windows(4)
        .filter(|window| window == &samx.as_slice())
        .count(); 
    forward + backtward
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

fn transpose (v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}