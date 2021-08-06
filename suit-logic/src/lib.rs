pub enum Action {
    Increment,
    Decrement,
}

pub struct Counter {
    pub value: isize,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
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
    pub fn execute(&mut self, a: Action) {
        match a {
            Action::Increment => self.increment(),
            Action::Decrement => self.decrement(),
        }
    }
}
