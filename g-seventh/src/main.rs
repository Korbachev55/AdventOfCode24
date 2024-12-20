use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
    let file_path = "data.txt";
    //let file_path = "example.txt";
    let start = Instant::now();

    // Open the file
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = io::BufReader::new(file);



    let mut collector = 0;


    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line

        let mut parts = line.split_whitespace();


        // 1) Take the target number 2) remove the colon 3) cast to i64.
        let mut target_str = parts.next().unwrap_or("");    
        target_str = &target_str[..target_str.len()-1];
        let target = target_str.parse::<i64>().unwrap();
    

        // Collect the rest as integers
        let integers: Vec<i64> = parts
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();


        collector += check_operations(target, integers);


    }
    
    let duration = start.elapsed();
    
    println!("Time taken: {:.2?}", duration);
    println!("Collector: {collector}");


    Ok(())
}

fn compute(integers: &[i64], operators: &[usize]) -> i64 {
    //println!("I{:?} O{:?}", integers, operators); 
    let mut new_integers = integers.to_vec();
    if integers.len() <= 1 { 
        integers[0]
    } else {
        if operators[0] == 0 {
            new_integers[1] = integers[0] + integers[1];
            //println!("+: {:?}", &new_integers[1..]);
        } else if operators[0] == 1 {
            new_integers[1] = integers[0] * integers[1];
            //println!("*: {:?}", &new_integers[1..]);
        } else if operators[0] == 2 {
            let mut concatenated = integers[0].to_string() + &integers[1].to_string();
            new_integers[1] = concatenated.parse::<i64>().unwrap();
            //println!("||: {:?}", &new_integers[1..]);
        }
        //println!("{:<width$} {:<width$} {:<width$}", integers[0], operators[0], integers[1], width=3);
        compute(&new_integers[1..], &operators[1..])
    }
}

fn check_operations(target: i64, integers: Vec<i64>) -> i64 {
    let n = integers.len() - 1;
    let mut generator = ConfigGenerator::new(n);

    for iter in 0..(3usize.pow(n as u32)+1) { // Iterate through all possible configurations
        if let Some(config) = generator.next() {
            //println!("Generator: {:?}", config);

            let equation_result = compute(&integers, &config[0..]);
            //println!("Compute: {}, target: {}\n", equation_result, target);
            if equation_result == target {
                return target;
            }
            
        }
    }
    0
}


use std::iter;

struct ConfigGenerator {
    n: usize,
    current: Vec<usize>,
    max: usize,
    first: bool,
}

impl ConfigGenerator {
    fn new(n: usize) -> Self {
        ConfigGenerator {
            n,
            current: vec![0; n],
            max: 3usize.pow(n as u32), // Total number of configurations
            first: true
        }
    }
}

impl Iterator for ConfigGenerator {
    type Item = Vec<usize>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if self.current.iter().all(|&x| x == 2) {
                let mut ret: Vec<usize> = iter::repeat(2).take(self.n).collect();
                //println!("A");
                self.first = false;
                return Some(ret); // We've exhausted all configurations
                //return None;
            }
        } else {
            //println!("B");
            return None;
        }

        let result = self.current.clone();

        // Increment the current configuration
        for i in (0..self.n).rev() {
            if self.current[i] < 2 {
                self.current[i] += 1;
                break;
            } else {
                self.current[i] = 0;
            }
        }

        Some(result)
    }
}