struct TwoVectors<T> {
    vec1: Vec<T>,
    vec2: Vec<T>,
}

impl<T> TwoVectors<T> {
    // Constructor
    fn new(vec1: Vec<T>, vec2: Vec<T>) -> Self {
        Self { vec1, vec2 }
    }

    // Add an element to the first vector
    fn add_to_vec1(&mut self, element: T) {
        self.vec1.push(element);
    }

    // Add an element to the second vector
    fn add_to_vec2(&mut self, element: T) {
        self.vec2.push(element);
    }

    // Get the length of the first vector
    fn len_vec1(&self) -> usize {
        self.vec1.len()
    }

    // Get the length of the second vector
    fn len_vec2(&self) -> usize {
        self.vec2.len()
    }

    // Retrieve immutable references to both vectors
    fn get_vectors(&self) -> (&Vec<T>, &Vec<T>) {
        (&self.vec1, &self.vec2)
    }
}

fn main() {
    // Create a new instance of TwoVectors
    let mut my_vectors = TwoVectors::new(vec![1, 2, 3], vec![4, 5]);

    // Add elements
    my_vectors.add_to_vec1(6);
    my_vectors.add_to_vec2(7);

    // Print lengths
    println!("Length of vec1: {}", my_vectors.len_vec1());
    println!("Length of vec2: {}", my_vectors.len_vec2());

    // Access both vectors
    let (vec1, vec2) = my_vectors.get_vectors();
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
}