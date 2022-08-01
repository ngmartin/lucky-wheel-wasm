mod browser;
mod item;
mod render_loop;
mod renderer;
mod utils;

use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use item::Items;
use render_loop::RenderLoop;
use renderer::Renderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct LuckyWheel {
    renderer: Rc<RefCell<Renderer>>,
    items: Rc<RefCell<Items>>,
}

#[wasm_bindgen]
impl LuckyWheel {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> LuckyWheel {
        utils::set_panic_hook();

        let items = Rc::new(RefCell::new(Items::new()));
        let renderer = Rc::new(RefCell::new(Renderer::new(canvas_id, items.clone())));
        renderer.borrow().draw(0.0);

        LuckyWheel { renderer, items }
    }

    pub fn start(&self) {
        let render_loop: Rc<RefCell<RenderLoop>> =
            Rc::new(RefCell::new(RenderLoop::new(self.renderer.clone())));

        render_loop.borrow_mut().closure = Some({
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || {
                render_loop.borrow_mut().render_loop();
            }))
        });

        let degree = self
            .items
            .borrow()
            .get_stoped_offset_degree(1)
            .expect("item id not found");
        render_loop.borrow_mut().start(degree);
    }
}
