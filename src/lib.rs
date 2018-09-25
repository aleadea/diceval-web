extern crate wasm_bindgen;
extern crate diceval;
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
pub use diceval::types::Num;


#[derive(Serialize, Deserialize, Debug)]
struct RollResult {
    value: Num,
    description: String,
}


#[wasm_bindgen]
pub fn roll(roll_command: &str) -> JsValue {
    if let Ok((value, description)) = diceval::eval(roll_command.to_string()) {
        JsValue::from_serde(&RollResult { value , description }).unwrap_or(JsValue::NULL)
    }
    else {
        JsValue::NULL
    }
}
