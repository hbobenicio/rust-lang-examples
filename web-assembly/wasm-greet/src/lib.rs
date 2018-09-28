extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Person {
    name: String,
    pub age: u8
}

#[wasm_bindgen]
impl Person {

    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age
        }
    }

    #[wasm_bindgen(js_name = getName)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(js_name = setName)]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn birthday(&mut self) {
        self.age += 1
    }
}
