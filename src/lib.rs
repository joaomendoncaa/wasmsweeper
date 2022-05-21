mod random;
mod sweeper;

use std::cell::RefCell;

use sweeper::Sweeper;
use wasm_bindgen::prelude::*;

thread_local! {
    static SWEEPER: RefCell<Sweeper> = RefCell::new(Sweeper::new(10, 10, 5));
}

#[wasm_bindgen(js_name = getBoardState)]
pub fn get_board_state() -> String {
    return SWEEPER.with(|sweeper| sweeper.borrow().to_string());
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
    SWEEPER.with(|sweeper| sweeper.borrow_mut().open((x, y)));
}
