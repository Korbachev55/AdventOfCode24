pub struct RuleOrder {
    number: String,
    before: Vec<String>,
    after: Vec<String>,
}

impl RuleOrder {
    pub fn new(number: String, before: Vec<String>, after: Vec<String>) -> Self {
        Self { number, before, after }
    }

    pub fn add_number(&mut self, element: String) {
        self.number = element;
    }  

    pub fn add_before(&mut self, element: String) {
        self.before.push(element);
    }

    pub fn add_after(&mut self, element: String) {
        self.after.push(element);
    }

    pub fn get_number(&self) -> &String {
        &self.number
    }

    pub fn get_before(&self, target: &str) -> bool {
        self.before.contains(&target)
    }

    pub fn get_after(&self, target: &str) -> bool {
        self.after.contains(&target)
    }
}


