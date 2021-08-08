use crate::log;
use js_sys::Function;
use suit_logic::AppClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebClient {
    update_view: Function,
    load_state: Function,
    save_state: Function,
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(options: JsValue) -> WebClient {
        // TODO figure this out
        WebClient {
            update_view: js_sys::Reflect::get(&options, &JsValue::from("update"))
                .unwrap()
                .into(),
            load_state: js_sys::Reflect::get(&options, &JsValue::from("loadState"))
                .unwrap()
                .into(),
            save_state: js_sys::Reflect::get(&options, &JsValue::from("saveState"))
                .unwrap()
                .into(),
        }
    }
}

impl AppClient for WebClient {
    fn update_view(&self, state: isize) {
        let this = JsValue::null();
        self.update_view
            .call1(&this, &JsValue::from(state as f64))
            .unwrap();
    }
    fn store_state(&self, state: isize) {
        let this = JsValue::null();
        self.save_state
            .call1(&this, &JsValue::from(state as f64))
            .unwrap();
    }
    fn load_state(&self) -> Option<isize> {
        let this = JsValue::null();
        let state = self.load_state.call0(&this).unwrap().as_string();

        if let Some(state) = state {
            Some(state.parse::<isize>().unwrap())
        } else {
            None
        }
    }
}
