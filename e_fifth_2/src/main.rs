use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() -> io::Result<()> {
    //let file_path = "data.txt";
    let file_path = "example.txt";

    // Open the file
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = io::BufReader::new(file);

    let collector: u16 = 0;
    let mut separator = false;


    let mut order: HashMap<String, (Vec<String>, Vec<String>)> = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line
        let mut rules: Vec<String> = vec![];
        let mut updates: Vec<String> = vec![];

        
        if line == "" {
            separator = true;
        } else if !separator {
            rules = line.split("|")
                        .filter_map(|s| s.parse::<String>().ok())
                        .collect();

            order.insert(rules[0].clone(), (vec![], vec![rules[1].clone()]));
            order.insert(rules[1].clone(), (vec![rules[0].clone()], vec![]));
            println!("r: {} {}", rules[0], rules[1]);
        } else {
            updates = line.split(",")
                          .filter_map(|s| s.parse::<String>().ok())
                          .collect();
            //println!("u: {:?}", updates);
        }
    }

    println!("O: {:?}", order);

    Ok(())
}

fn add_after()