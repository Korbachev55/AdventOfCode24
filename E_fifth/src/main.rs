use std::fs::File;
use std::io::{self, BufRead};
//use std::collections::HashMap;

use e_fifth::RuleOrder;

fn main() -> io::Result<()> {

    //let file_path = "data.txt";
    let file_path = "example.txt";

    let order: Vec<RuleOrder> = read_rules(file_path).expect("No rules!");

    //let mut collector: u16 = 0;

    
    let mut i = 0;
    for number in &order {
        println!("Number: {} {} {:?} {:?}", i, number.get_number(), number.get_before(), number.get_after());
        i += 1;
    }

    Ok(())
}

fn read_rules(file_path: &str) -> io::Result<Vec<RuleOrder>> {


    // Open the file
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = io::BufReader::new(file);


    // Choose the data structure
    //let mut test: RuleOrder = RuleOrder::new(number(), vec![], vec![]);
    let mut order: Vec<RuleOrder> = vec![];

    // These things
    let mut _rules: Vec<String> = vec![];


    // The file separates information with an empty line
    let mut separator = false;


    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line

        if line == "" {
            separator = true;
        } else if !separator {
            _rules = line.split("|")
                        .filter_map(|s| s.parse::<String>().ok())
                        .collect();

            // Adding numbers
            //println!("r: {} {}", _rules[0], _rules[1]);
            if order.iter().any(|order| order.get_number() == &_rules[0]) {
                println!("R: {}", &_rules[0]);
            } else {    
                println!("R: adding {}", &_rules[0]);
                order.push(RuleOrder::new(_rules[0].clone(), vec![], vec![]));
            }    
            if order.iter().any(|order| order.get_number() == &_rules[1]) {
                println!("L: {}", &_rules[1]);
            } else {
                println!("L: adding {}", &_rules[1]);
                order.push(RuleOrder::new(_rules[1].clone(), vec![], vec![]));
            }

            // Adding the other number into the one of the lists
            println!("r: {} {}", _rules[0], _rules[1]);
            //if order.iter().any(|order| order.get_number() == &_rules[0])
            if obj.contains_number(target) {
                println!("The vector contains the number {}", target);
            } else {
                println!("The vector does not contain the number {}", target);
            }            
            println!("");


        }
    }
    Ok(order)
}

/* 
fn read_updates(file_path: &str) -> i8 {
    // Open the file
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = io::BufReader::new(file);

    let mut updates: Vec<String> = vec![];

    if line == "" {
        separator = true;
    } else {
        updates = line.split(",")
                        .filter_map(|s| s.parse::<String>().ok())
                        .collect();
        //println!("u: {:?}", updates);
    }
}
*/