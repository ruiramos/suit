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
    update_view_fn: Function,
}

#[wasm_bindgen]
impl CounterApp {
    pub fn new(update_view_fn: Function) -> CounterApp {
        let app = CounterApp {
            counter: Counter::new(),
            update_view_fn,
        };
        CounterApp::update_view(&app);
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
            self.update_view();
        }
    }

    fn update_view(&self) {
        let this = JsValue::null();
        self.update_view_fn
            .call1(&this, &JsValue::from(self.counter.get_value().to_string()))
            .unwrap();
    }
}
