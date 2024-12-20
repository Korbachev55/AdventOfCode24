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
                println!("A");
                self.first = false;
                return Some(ret); // We've exhausted all configurations
                //return None;
            }
        } else {
            println!("B");
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
/* 
fn compute(integers: &[i16], operators: &[usize]) -> i64 {
    if operators.len() <= 0 { 
        integers[0]
    } else {
        if 
        compute(&new_integers[1..], &operators[1..])
}
 */
fn main() {
    let n = 3; // Length of the vector
    let mut generator = ConfigGenerator::new(n);

    //let mut ret: Vec<usize> = iter::repeat(2).take(n).collect();
    //println!("ret: {:?}", ret);

    // Generate the first 10 configurations as an example
    for _ in 0..100 {
        if let Some(config) = generator.next() {
            //println!("{:?}", config);
        } else { break }
    }

    let numbers = vec![1, 2, 3, 4];
    let operators = vec![0, 1, 0];


    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Vec: {:?}", numbers);
    println!("The sum is: {}", sum);



    // Compute the result based on operators
    let result = numbers
        .iter()
        .zip(operators.iter()) // Pair each value with its corresponding operators
        .fold(0, |acc, (&value, &operator)| {
            println!("A:{acc} V:{value} O:{operator}");
            if operator == 0 {
                acc + value
            } else {
                acc * value
            } 
        });


    println!("The computed result is: {}", result);

}