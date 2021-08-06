use js_sys::Function;
use suit_logic::{Action, Counter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    log("running start");
}

#[wasm_bindgen]
pub struct CounterApp {
    counter: Counter,
    update_fn: Function,
}

#[wasm_bindgen]
impl CounterApp {
    pub fn new(update_fn: Function) -> CounterApp {
        let app = CounterApp {
            counter: Counter::new(),
            update_fn,
        };
        CounterApp::call_update(&app);
        app
    }

    pub fn dispatch(&mut self, action: &str) {
        let action = match action {
            "inc" => Some(Action::Increment),
            "dec" => Some(Action::Decrement),
            _ => {
                log(&format!("unknown action: {}", action));
                None
            }
        };

        if let Some(a) = action {
            self.counter.execute(a);
            self.call_update();
        }
    }

    fn call_update(&self) {
        let this = JsValue::null();
        self.update_fn
            .call1(&this, &JsValue::from(self.counter.get_value().to_string()))
            .unwrap();
    }
}
