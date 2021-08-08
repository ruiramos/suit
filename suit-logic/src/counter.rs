pub struct Counter {
    value: isize,
}

impl Counter {
    pub fn new(initial_value: Option<isize>) -> Counter {
        Counter {
            value: initial_value.unwrap_or(0),
        }
    }
    pub fn increment(&mut self) {
        self.value += 1;
    }
    pub fn decrement(&mut self) {
        self.value -= 1;
    }
    pub fn get_value(&self) -> isize {
        self.value
    }
}
