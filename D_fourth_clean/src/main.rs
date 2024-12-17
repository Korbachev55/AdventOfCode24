use std::fs::File;
use std::{isize};
use std::io::{self, BufRead};



fn main() -> io::Result<()> {
    // Open the file
    //let file = File::open("small.txt")?;
    let file = File::open("data.txt")?;
    let reader = io::BufReader::new(file);

    // Initialize a matrix to store the characters
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line
        let row: Vec<char> = line.chars().collect(); // Convert the line to a Vec<char>
        matrix.push(row);
    }




    
    //println!("Validate: {}", validate(&matrix, 0, 0));
    //println!("Horizontal: {:?}", horizontal(&matrix, 1, 1));
    //println!("Validate: {:?}", validate('X', 'M', 'A', 'S'));
    //println!("Validate: {:?}", validate('S', 'A', 'M', 'X'));

/* 
    let mut accumulator: usize = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j , &val) in row.iter().enumerate() {
            println!("matrix[{}][{}] = {}", i, j, val);
            if horizontal(&matrix, i, j) {
                println!("Horizontal");
                accumulator += 1;
            }
            if vertical(&matrix, i, j) {
                accumulator += 1;
                println!("Vertical");
            }
            if diagonal(&matrix, i, j) {
                accumulator += 1;
                println!("Diagonal");
            }
            if other_diagonal(&matrix, i, j) {
                accumulator += 1;
                println!("Diagonal");
            }                                 
        }
        println!("Accumulator {}", accumulator);
        //println!();
    }
     */

    let mut x_accumulator: usize = 0;
    let mut top_left: char = 'O';
    let mut top_right: char = 'O';
    let mut bottom_left: char = 'O';
    let mut bottom_right: char = 'O';
    let mut complete: bool = true;
    for (j, row) in matrix.iter().enumerate() {

        for (i , &val) in row.iter().enumerate() {
            complete = true;
            if val == 'A' {
                match get_char(&matrix, (i as isize)-1, (j as isize)+1) {
                    None               => complete = false,
                    Some(letter) => top_left = letter,
                }
                match get_char(&matrix, (i as isize)+1, (j as isize)+1) {
                    None               => complete = false,
                    Some(letter) => top_right = letter,
                }
                match get_char(&matrix, (i as isize)-1, (j as isize)-1) {
                    None               => complete = false,
                    Some(letter) => bottom_left = letter,
                }
                match get_char(&matrix, (i as isize)+1, (j as isize)-1) {
                    None               => complete = false,
                    Some(letter) => bottom_right = letter,
                }
                if complete {
                    println!("matrix[{}][{}] = {}\nX-Accumulator: {}", i, j, val, x_accumulator);
                    println!("T: {} {}\nB: {} {}\n", top_left, top_right, bottom_left, bottom_right);
                    if top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M' {
                        //x_accumulator += 1;
                        if top_right == 'M' && bottom_left == 'S' || top_right == 'S' && bottom_left == 'M' {
                            x_accumulator += 1;
                        }                
                    }
                }
            }                               
        }
        //println!();
    }

    println!("x: {}", x_accumulator);
    //print_matrix(&matrix);
    
    Ok(())
}


fn get_char(matrix: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    if y < 0 || x < 0 {
        return None;
    }
    if let Some(row) = matrix.get(y as usize) {
        if let Some(&letter) = row.get(x as usize) {
            Some(letter)
        } else { None }
    } else { None }
}

fn horizontal(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut letters = vec!['O', 'O', 'O', 'O'];
    let mut complete: bool = true;
    for iter in 0..4 {
        match get_char(matrix, (x as isize)+(iter as isize) , y as isize) {
            None               => complete = false,   
            Some(letter) => { letters[iter] = letter; },
        }
    }
    //println!("letters {:?}", letters);
    if complete {
        validate(letters[0], letters[1], letters[2], letters[3])
    } else {
        false
    }
}

fn vertical(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut letters = vec!['O', 'O', 'O', 'O'];
    let mut complete: bool = true;
    for iter in 0..4 {
        match get_char(matrix, x as isize, (y as isize)+(iter as isize)) {
            None               => complete = false,   
            Some(letter) => { letters[iter] = letter; },
        }
    }
    //println!("letters {:?}", letters);
    if complete {
        validate(letters[0], letters[1], letters[2], letters[3])
    } else {
        false
    }
}

fn diagonal(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut letters = vec!['O', 'O', 'O', 'O'];
    let mut complete: bool = true;
    for iter in 0..4 {
        match get_char(matrix, (x as isize)+(iter as isize), (y as isize)+(iter as isize)) {
            None               => complete = false,   
            Some(letter) => { letters[iter] = letter; },
        }
    }
    //println!("letters {:?}", letters);
    if complete {
        validate(letters[0], letters[1], letters[2], letters[3])
    } else {
        false
    }
}

fn other_diagonal(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut letters = vec!['O', 'O', 'O', 'O'];
    let mut complete: bool = true;
    for iter in 0..4 {
        match get_char(matrix, (x as isize)-(iter as isize), (y as isize)+(iter as isize)) {
            None               => complete = false,   
            Some(letter) => { letters[iter] = letter; },
        }
    }
    //println!("letters {:?}", letters);
    if complete {
        validate(letters[0], letters[1], letters[2], letters[3])
    } else {
        false
    }
}

fn validate(_0: char, _1: char, _2: char, _3: char) -> bool {
    if _0 == 'X' && _1 == 'M' && _2 == 'A' && _3 =='S' || _0 == 'S' && _1 == 'A' && _2 == 'M' && _3 =='X' {
        true
    } else {
        false
    }
}



fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        println!("{}", row.iter().collect::<String>());
    }
}

