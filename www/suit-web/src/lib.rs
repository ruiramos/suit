use suit_logic::{Action, App};
use wasm_bindgen::prelude::*;

mod webclient;
use crate::webclient::WebClient;

#[wasm_bindgen(start)]
pub fn main() {
    log("running start");
    init_panic_hook();
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct CounterApp {
    app: App,
}

#[wasm_bindgen]
impl CounterApp {
    pub fn new(client: WebClient) -> CounterApp {
        CounterApp {
            app: App::new(Box::new(client)),
        }
    }
    pub fn increment(&mut self) {
        self.app.dispatch(Action::Increment);
    }
    pub fn decrement(&mut self) {
        self.app.dispatch(Action::Decrement);
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
