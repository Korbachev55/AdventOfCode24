
use std::fs;

fn main() -> std::io::Result<()> {
    let file_path = "data.txt";
    //let file_path = "example.txt";
    let contents = fs::read_to_string(file_path)?;
    //println!("File:\n{}", contents);
    let mut accumulator = 0;
    let mut active: bool = true;

    for index in 0..contents.len()-12 {

        let slice = &contents[index..index+12];
        let mul_slice = &contents[index..index+4];
        let do_slice: &str = &contents[index..index+4];
        let dont_slice: &str = &contents[index..index+7];

        if do_slice == "do()" {
            //println!("slice: {slice}");
            active = true;
        } else if dont_slice == "don't()" {
            //println!("slice: {slice}");
            active = false;
        }


        //println!("Whole: {slice}");
        if mul_slice == "mul(" {
            loop {
                if active == false { break }

                let mut open_slice = String::new();
                let mut before_comma = String::new();
                let mut close_slice = String::new();

                let mut open_i: usize = 0;
                let mut comma_i: usize = 0;
                let mut close_i: usize = 0;
                

                if let Some(open_index) = slice.find('(') {
                    open_i = open_index + 1;
                    open_slice = (&slice[..open_index]).to_string();                  
                }
                if let Some(comma_index) = slice.find(',') {
                    comma_i = comma_index + 1;
                    before_comma = (&slice[..comma_index]).to_string();
                } else { break }
                if let Some(close_index) = slice.find(')') {
                    close_i = close_index + 1;
                    close_slice = (&slice[..close_index]).to_string();
                } else { break }

/* 
                let mut int1_len = 0;
                let mut int2_len = 0;

                if ((comma_i as i64) - (open_i as i64)) <= 4 {
                    int1_len = (comma_i as i64) - (open_i as i64);
                    //println!("Sub: {int1_len}");
                    //println!("Before comma: {}", before_comma);
                } else { break }
                if ((close_i as i64) - (comma_i as i64)) <= 4 {
                    int2_len = (close_i as i64) - (comma_i as i64);
                    //println!("Sub: {int2_len}");
                    //println!("Before close: {}", close_slice);
                } else { break }     
                            */
                
                //println!("Indices: {} {} {}", open_i, comma_i, close_i);
                //println!("Ints: {} {}", &slice[open_i..comma_i-1], &slice[comma_i..close_i-1]);

                let mut int1 = 0;
                let mut int2 = 0;

                match &slice[open_i..comma_i-1].parse::<i64>() {
                    Ok(value) => int1 = *value,
                    Err(_) => break,
                }
                match &slice[comma_i..close_i-1].parse::<i64>() {
                    Ok(value) => int2 = *value,
                    Err(_) => break,
                }                
                println!("{} {}", int1, int2);
                accumulator += int1 * int2;
                println!("Acc: {accumulator}");
                println!();

                break
            }

        }


    }


    Ok(())
}