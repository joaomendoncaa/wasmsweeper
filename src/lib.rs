mod random;
mod sweeper;

use sweeper::Sweeper;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(js_name = getBoardState)]
pub fn get_board_state() -> String {
    let sweeper = Sweeper::new(10, 10, 5);

    return sweeper.to_string();
}
