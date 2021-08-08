mod counter;
use crate::counter::Counter;

pub enum Action {
    Increment,
    Decrement,
}

pub trait AppClient {
    // TODO abstract state
    fn update_view(&self, state: isize);
    fn store_state(&self, state: isize);
    fn load_state(&self) -> Option<isize>;
}

pub struct App {
    counter: Counter,
    client: Box<dyn AppClient>,
}

impl App {
    pub fn new(client: Box<dyn AppClient>) -> App {
        let counter = Counter::new(client.load_state());
        client.update_view(counter.get_value());
        App { client, counter }
    }
    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::Increment => self.counter.increment(),
            Action::Decrement => self.counter.decrement(),
        }
        let state = self.counter.get_value();
        self.client.update_view(state);
        self.client.store_state(state);
    }
}
