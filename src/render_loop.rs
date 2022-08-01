use crate::browser;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const MAX_VELOCITY: f64 = 6.0;
const DURATION: u8 = 6;
const FPS: u8 = 60;

pub struct RenderLoop {
    renderer: Rc<RefCell<Renderer>>,
    degree: f64,
    animation_id: Option<i32>,
    pub closure: Option<Closure<dyn FnMut()>>,
    tick: u32,
    half_second: u32,
    velocity: f64,
    stoped_degree: f64,
    is_stopping: bool,
}

impl RenderLoop {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RenderLoop {
        RenderLoop {
            renderer,
            closure: None,
            animation_id: None,
            degree: 0.0,
            tick: 0,
            half_second: 0,
            velocity: MAX_VELOCITY,
            stoped_degree: 0.0,
            is_stopping: false,
        }
    }

    pub fn render_loop(&mut self) {
        if !self.is_stopping {
            self.velocity = ((DURATION * 2) as u32 - self.half_second) as f64 * MAX_VELOCITY
                / (DURATION * 2) as f64;

            if self.velocity < 0.5 {
                self.velocity = 0.5;
                self.is_stopping = true;
            }
        } else {
            if self.degree.floor() == self.stoped_degree.floor() {
                self.velocity = 0.0;
                self.stop();
            }
        }

        self.renderer.borrow().draw(self.degree);
        self.add_degree(self.velocity);

        self.animation_id = if let Some(ref closure) = self.closure {
            Some(browser::request_animation_frame(closure))
        } else {
            None
        };

        self.tick();
    }

    pub fn start(&mut self, stoped_degree: f64) {
        self.stoped_degree = stoped_degree;
        self.velocity = MAX_VELOCITY;
        self.half_second = 0;
        self.is_stopping = false;

        self.render_loop();
    }

    fn stop(&mut self) {
        if let Some(id) = self.animation_id {
            browser::cancel_animation_frame(id);
            self.animation_id = None;
        }
    }

    fn add_degree(&mut self, degree: f64) {
        self.degree = (self.degree + degree) % 360.0;
    }

    fn tick(&mut self) {
        self.tick += 1;

        if self.tick >= (FPS / 2).into() {
            self.tick = 0;
            self.half_second += 1;
        }
    }
}
