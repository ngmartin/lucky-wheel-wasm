use crate::browser;
use crate::renderer::Renderer;
use crate::spinning::Spinning;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const MAX_VELOCITY: f64 = 15.0;
const MIN_VELOCITY: f64 = 0.2;
const CIRCLES: u32 = 6 * 360;

pub struct RenderLoop {
    spinning: Spinning,
    renderer: Rc<RefCell<Renderer>>,
    animation_id: Option<i32>,
    pub closure: Option<Closure<dyn FnMut()>>,
}

impl RenderLoop {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RenderLoop {
        let spinning = Spinning::new(MIN_VELOCITY, MAX_VELOCITY, CIRCLES);

        RenderLoop {
            spinning,
            renderer,
            closure: None,
            animation_id: None,
        }
    }

    pub fn render_loop(&mut self) {
        self.spinning.tick();

        if self.spinning.is_stop() {
            self.stop();
            return;
        }

        self.renderer.borrow().draw(self.spinning.degree());
        self.animation_id = if let Some(ref closure) = self.closure {
            Some(browser::request_animation_frame(closure))
        } else {
            None
        }
    }

    pub fn start(&mut self, stoped_degree: u32) {
        self.spinning.start(stoped_degree);
        self.render_loop();
    }

    fn stop(&mut self) {
        if let Some(id) = self.animation_id {
            browser::cancel_animation_frame(id);
            self.animation_id = None;
        }
    }
}
