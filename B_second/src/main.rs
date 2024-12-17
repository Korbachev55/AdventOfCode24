use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;

fn check_order(slice: &[i32]) -> bool {


    // Check if the slice is sorted in ascending order
    let is_ascending = slice.windows(2).all(|w| w[0] <= w[1]);
    // Check if the slice is sorted in descending order
    let is_descending = slice.windows(2).all(|w| w[0] >= w[1]);

    let is_same = slice.windows(2).all(|w| w[0] == w[1]);

    //let too_different = slice.windows(2).all(|w| (w[0] - w[1]).abs() >= 1);
    //println!("dif: {}", slice.windows(2));

    let range_condition = slice.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        diff >= 1 && diff <= 3
    });
    
    if range_condition {
    
        if is_ascending {
            println!("ascending");
            return true
        } else if is_descending {
            println!("descending");
            return true
        } else {
            println!("error");
        }
        
    }
    false
}


fn check_order_2(slice: &[i32]) -> bool {

    let _report_length = (slice.len() as i32) - 1;

    for num in slice {
        println!("Num {}", num);
    }

    /* 
    for iter in 00..(slice.len() as i32)-1 {

        println!("Num {}", slice[iter]);
    }   
    */

    false
}


fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "data.txt";
    //let file_path = "short.txt";

    // Open the file
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = io::BufReader::new(file);



    let mut collector = 0;
    let mut collector_2 = 0;

    // Read the file line by line
    for line in reader.lines() {
        let mut report: Vec<i32> = Vec::new();
        let line = line?; // Unwrap the Result to get the line

        // Split the line into parts and parse the integers
        let _iter = line
            .split_whitespace()
            .filter_map(|word| word.parse::<i32>().ok());

        //println!("Res: {:?}", line);

        for number in _iter {
            //println!("Num: {}", number);
            report.push(number);

        }    

        if check_order(&report) {
            collector += 1;
        }
        
        // Part 2:
        let mut all_ok = true;
        for index in 0..report.len() {
            let mut partial_report: Vec<i32> = report.clone();
            partial_report.remove(index);
            println!("{:?}", partial_report);

            if check_order(&partial_report) {
                all_ok = false;
            } 
        }


        if all_ok == false {
            collector_2 += 1;
        }

        println!("Collector {}", collector_2);




    }


    Ok(())
}