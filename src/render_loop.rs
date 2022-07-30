use crate::browser;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct RenderLoop {
    renderer: Rc<RefCell<Renderer>>,
    degree: f64,
    pub closure: Option<Closure<dyn FnMut()>>,
}

impl RenderLoop {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RenderLoop {
        RenderLoop {
            renderer,
            closure: None,
            degree: 0.0,
        }
    }

    pub fn render_loop(&mut self) {
        self.renderer.borrow().draw(self.degree);
        self.degree = (self.degree + 2.0) % 360.0;

        if let Some(ref closure) = self.closure {
            Some(browser::request_animation_frame(closure));
        }
    }
}
