mod browser;
mod renderer;
mod utils;

use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use renderer::Renderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct LuckyWheel {
    renderer: Renderer,
}

#[wasm_bindgen]
impl LuckyWheel {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> LuckyWheel {
        let renderer = Renderer::new(canvas_id);
        renderer.draw(0);

        LuckyWheel { renderer }
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    let mut degree = 0;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        degree += 1;
        web_sys::console::log_1(&JsValue::from(degree));

        browser::request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    browser::request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
