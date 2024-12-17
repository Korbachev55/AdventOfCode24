use std::fs::File;
use std::io::{self, BufRead};



fn count_occurrences(vec: &Vec<i32>, target: i32) -> usize {
    vec.iter().filter(|&&x| x == target).count()
}

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "numbers.txt";

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Create vectors for each column
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line

        // Split the line into parts and parse the integers
        let mut iter = line.split_whitespace();
        if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                column1.push(num1);
                column2.push(num2);
            }
        }

    }

    column1.sort();
    column2.sort();

    let mut collector: i32 = 0;
    let mut difference: i32 = 0;
    for i in 0 .. column1.len() {
        difference = column1[i] - column2[i];
        collector += difference.abs();
    }



    // Print the vectors
    //println!("Column 1: {:?}", column1);
    //println!("Column 2: {:?}", column2);
    println!("collector: {:?}", collector);
/* 
    let mut second_collector: usize = 0;
    for num2: i32 in 0 .. column1.len() {
        second_collector += &count_occurrences(&column2, num2)
    }
 */
    println!("occurances: {}", 3);

    Ok(())
}